#![cfg_attr(not(feature = "libos"), no_std)]
// #![deny(warnings)]
#![no_main]
#![feature(naked_functions, asm_sym, asm_const)]
#![feature(default_alloc_error_handler)]

use core::sync::atomic::{AtomicBool, Ordering};

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

    use alloc::boxed::Box;

    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    unsafe{
        info!("enable");
        use core::arch::asm;
        use riscv::register::sstatus;
        use riscv::register::sie;
        unsafe {
            sie::set_stimer();
            sie::set_sext();
            sstatus::set_sie();
        }
    }

    
    //register irq for nvme
    // let irq = kernel_hal::drivers::all_irq().find("riscv-plic").unwrap();
    // let nvme1 = kernel_hal::drivers::all_block()
    // .find("nvme")
    // .unwrap();
    // let irq_num = 0x21;
    // irq.register_handler(irq_num, Box::new(move || nvme1.handle_irq(irq_num)));
    // irq.unmask(irq_num);
    
    
    let irq = kernel_hal::drivers::all_irq().find("riscv-plic").unwrap();
    let nvme = kernel_hal::drivers::all_block().find("nvme").unwrap();
    let irq_num = 0x21;
    let _r = irq.register_handler(irq_num, Box::new(move || nvme.handle_irq(irq_num)));
    let _r = irq.unmask(irq_num);
    
    let intc = kernel_hal::drivers::all_irq().find("riscv-intc-cpu0").unwrap();

    intc.register_handler(9, Box::new(move || irq.handle_irq(9)));
    intc.unmask(9);
    
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

    // loop {

    //     let mut queue = USER_TASK_QUEUE.lock();
    //     let task = queue.peek_task();
    //     match task {
    //         // have any task
    //         Some(task) => {
    //             let mywaker = task.clone();
    //             let waker = waker_ref(&mywaker);
    //             let mut context = Context::from_waker(&*waker);

    //             let r = task.reactor.clone();
    //             let mut r = r.lock();

    //             if r.is_ready(task.id) {
    //                 let mut future = task.future.lock();
    //                 match future.as_mut().poll(&mut context) {
    //                     Poll::Ready(_) => {
    //                         // 任务完成
    //                         r.finish_task(task.id);
    //                     }
    //                     Poll::Pending => {
    //                         r.add_task(task.id);
    //                     }
    //                 }
    //             } else if r.contains_task(task.id) {
    //                 r.add_task(task.id);
    //             } else {
    //                 let mut future = task.future.lock();
    //                 match future.as_mut().poll(&mut context) {
    //                     Poll::Ready(_) => {
    //                         // // 任务完成
    //                         // println!("task completed");
    //                     }
    //                     Poll::Pending => {
    //                         r.register(task.id);
    //                     }
    //                 }
    //             }
    //         }
    //         None => return
    //     }
    // };
    // warn!("read_buf: {:?}", read_buf);



    // loop{

    // }
    cfg_if! {
        if #[cfg(all(feature = "linux", feature = "zircon"))] {
            panic!("Feature `linux` and `zircon` cannot be enabled at the same time!");
        } else if #[cfg(feature = "linux")] {
            let args = options.root_proc.split('?').map(Into::into).collect(); // parse "arg0?arg1?arg2"
            let envs = alloc::vec!["PATH=/usr/sbin:/usr/bin:/sbin:/bin".into()];
            let rootfs = fs::rootfs();
            let proc = zcore_loader::linux::run(args, envs, rootfs);
            utils::wait_for_exit(Some(proc))
        } else if #[cfg(feature = "zircon")] {
            let zbi = fs::zbi();
            let proc = zcore_loader::zircon::run_userboot(zbi, &options.cmdline);
            utils::wait_for_exit(Some(proc))
        } else {
            panic!("One of the features `linux` or `zircon` must be specified!");
        }
    }
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

async fn nvme_test(){
    info!("hello world");

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
    let _r = nvme_block.async_write_block(0, &buf1);


    // loop{
    //     match _r.poll{
    //         Poll::Ready(_) => {
    //             info!("write done");
    //             break;
    //         }
    //         Poll::Pending => {
    //             info!("write pending");
    //         }
    //     }
    // }
    // let mut read_buf = [0u8; 512];
    // let _r = nvme_block.async_read_block(0, &mut read_buf);

    // loop{
    //     match _r.poll{
    //         Poll::Ready(_) => {
    //             info!("write done");
    //             break;
    //         }
    //         Poll::Pending => {
    //             info!("write pending");
    //         }
    //     }
    // }

    // _r.await;

    // let buf2:&[u8] = &[2u8;512];
    // let _r = nvme_block.async_write_block(1, &buf2);

    // loop{
    //     match _r.poll{
    //         Poll::Ready(_) => {
    //             info!("write done");
    //             break;
    //         }
    //         Poll::Pending => {
    //             info!("write pending");
    //         }
    //     }
    // }

    // let mut read_buf = [0u8; 512];
    // let _r = nvme_block.async_read_block(1, &mut read_buf);

    // loop{
    //     match _r.poll{
    //         Poll::Ready(_) => {
    //             info!("write done");
    //             break;
    //         }
    //         Poll::Pending => {
    //             info!("write pending");
    //         }
    //     }
    // }

}