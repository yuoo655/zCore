#![cfg_attr(not(feature = "libos"), no_std)]
// #![deny(warnings)]
#![no_main]
#![feature(naked_functions, asm_sym, asm_const)]
#![feature(default_alloc_error_handler)]

#![feature(const_option)]

use core::sync::atomic::{AtomicBool, Ordering};
use alloc::boxed::Box;
use kernel_hal::drivers::scheme::BlockScheme;

extern crate alloc;
#[macro_use]
extern crate log;
#[macro_use]
extern crate cfg_if;

#[macro_use]
mod logging;

#[cfg(not(feature = "libos"))]
mod lang;

mod fs;
mod handler;
mod platform;
mod utils;

cfg_if! {
    if #[cfg(target_arch = "x86_64")] {
        #[path = "memory_x86_64.rs"]
        mod memory;
    } else {
        mod memory;
    }
}

static STARTED: AtomicBool = AtomicBool::new(false);

#[cfg(all(not(any(feature = "libos")), feature = "mock-disk"))]
static MOCK_CORE: AtomicBool = AtomicBool::new(false);

fn primary_main(config: kernel_hal::KernelConfig) {
    logging::init();
    memory::init();
    kernel_hal::primary_init_early(config, &handler::ZcoreKernelHandler);
    let options = utils::boot_options();
    logging::set_max_level(&options.log_level);
    info!("Boot options: {:#?}", options);
    memory::insert_regions(&kernel_hal::mem::free_pmem_regions());
    kernel_hal::primary_init();
    STARTED.store(true, Ordering::SeqCst);

    // test();

    nvme_test();

    panic!("end");
}

#[cfg(not(any(feature = "libos", target_arch = "aarch64")))]
fn secondary_main() -> ! {
    while !STARTED.load(Ordering::SeqCst) {
        core::hint::spin_loop();
    }
    kernel_hal::secondary_init();
    info!("hart{} inited", kernel_hal::cpu::cpu_id());
    #[cfg(feature = "mock-disk")]
    {
        if MOCK_CORE
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
        {
            utils::mock_disk();
        }
    }
    utils::wait_for_exit(None)
}


use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll, Waker},
};

// use core::task::{Wake};

use core::future::poll_fn;

use core::task::{RawWaker, RawWakerVTable};

use alloc::sync::Arc;
use alloc::vec::Vec;

fn test(){
    use alloc::boxed::Box;
    let irq = kernel_hal::drivers::all_irq().find("riscv-plic").unwrap();
    let nvme = kernel_hal::drivers::all_block().find("nvme").unwrap();
    let irq_num = 33;
    let _r = irq.register_handler(irq_num, Box::new(move || nvme.handle_irq(irq_num)));

    let _r = irq.unmask(irq_num);

    let nvme_block = kernel_hal::drivers::all_block()
    .find("nvme")
    .unwrap();

    let buf1:&[u8] = &[1u8;512];
    let _r = nvme_block.write_block(0, &buf1);
    warn!("r {:?}", _r);
    let mut read_buf = [0u8; 512];
    let _r = nvme_block.read_block(0, &mut read_buf);
    warn!("read_buf: {:?}", read_buf);

    let buf2:&[u8] = &[2u8;512];
    let _r = nvme_block.write_block(1, &buf2);
    warn!("r {:?}", _r);
    let mut read_buf = [0u8; 512];
    let _r = nvme_block.read_block(1, &mut read_buf);
    warn!("read_buf: {:?}", read_buf);
}

fn nvme_test(){
    let irq = kernel_hal::drivers::all_irq().find("riscv-plic").unwrap();
    let nvme = kernel_hal::drivers::all_block().find("nvme").unwrap();
    let irq_num = 0x21;
    let _r = irq.register_handler(irq_num, Box::new(move || nvme.handle_irq(irq_num)));
    let _r = irq.unmask(irq_num);
    let nvme_block = kernel_hal::drivers::all_block().find("nvme").unwrap();

    drop(irq);
    static buf1:&[u8] = &[1u8;512];
    unsafe{
        let mut f1 = nvme_block.async_write_block(0, &buf1);
        let wake1 =  MyWaker {};
        let mywaker = Arc::new(wake1);
        let waker = mywaker_into_waker(Arc::into_raw(mywaker));
        let mut cx = Context::from_waker(&waker);
        loop {
            match Future::poll(f1.as_mut(), &mut cx) {
                Poll::Ready(()) => {
                    break
                }
                Poll::Pending => {
                    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
                    unsafe{
                        use riscv::register::sstatus;
                        use riscv::register::sie;
                        unsafe {
                            sie::set_sext();
                            sstatus::set_sie();
                        }
                    }
                },
            };
        }
    }

    static buf2:&[u8] = &[2u8;512];
    unsafe{
        let mut f2 = nvme_block.async_write_block(1, &buf2);
        let wake2 =  MyWaker {};
        let mywaker = Arc::new(wake2);
        let waker = mywaker_into_waker(Arc::into_raw(mywaker));
        let mut cx = Context::from_waker(&waker);
        loop { 
            match Future::poll(f2.as_mut(), &mut cx) {
                Poll::Ready(()) => {
                    break
                }
                Poll::Pending => {
                    
                },
            };
        }
    }    

    static mut read_buf:[u8; 512] = [0u8; 512];
    unsafe{
        let mut f3 = nvme_block.async_read_block(0, &mut read_buf);
        let wake3 =  MyWaker {};
        let mywaker = Arc::new(wake3);
        let waker = mywaker_into_waker(Arc::into_raw(mywaker));
        let mut cx = Context::from_waker(&waker);
        loop { 
            match Future::poll(f3.as_mut(), &mut cx) {
                Poll::Ready(()) => {
                    break
                }
                Poll::Pending => {
                    
                },
            };
        }
        info!("read_buf: {:?}", read_buf);
    }  
    

    unsafe{
        let mut f4 = nvme_block.async_read_block(1, &mut read_buf);
        let wake4 =  MyWaker {};
        let mywaker = Arc::new(wake4);
        let waker = mywaker_into_waker(Arc::into_raw(mywaker));
        let mut cx = Context::from_waker(&waker);
        loop { 
            match Future::poll(f4.as_mut(), &mut cx) {
                Poll::Ready(()) => {
                    break
                }
                Poll::Pending => {
                    
                },
            };
        }
        info!("read_buf: {:?}", read_buf);
    }    
}



const VTABLE: RawWakerVTable = unsafe {
    RawWakerVTable::new(
        |s| mywaker_clone(&*(s as *const MyWaker)),   // clone
        |s| mywaker_wake(&*(s as *const MyWaker)),    // wake
        |s| {
            info!("wake_by_ref")
        }, 
        |s| drop(Arc::from_raw(s as *const MyWaker)), // decrease refcount
    )
};

#[derive(Clone)]
struct MyWaker {
}

fn mywaker_wake(s: &MyWaker) {
    let waker_arc = unsafe { Arc::from_raw(s) };
}

fn mywaker_clone(s: &MyWaker) -> RawWaker {
    let arc = unsafe { Arc::from_raw(s) };
    core::mem::forget(arc.clone()); // increase ref count
    RawWaker::new(Arc::into_raw(arc) as *const (), &VTABLE)
}


fn mywaker_into_waker(s: *const MyWaker) -> Waker {
    let raw_waker = RawWaker::new(s as *const (), &VTABLE);
    unsafe { Waker::from_raw(raw_waker) }
}
