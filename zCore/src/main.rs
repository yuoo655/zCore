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




    
    info!("hello world");
    nvme_test();



    loop{
        
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

// use core::task::{Wake};

use core::future::poll_fn;

use core::task::{RawWaker, RawWakerVTable};

use alloc::sync::Arc;


fn nvme_test(){

    info!("hello world");

    let irq = kernel_hal::drivers::all_irq().find("riscv-plic").unwrap();
    let nvme = kernel_hal::drivers::all_block().find("nvme").unwrap();
    let irq_num = 0x21;
    let _r = irq.register_handler(irq_num, Box::new(move || nvme.handle_irq(irq_num)));
    let _r = irq.unmask(irq_num);
    
    let nvme_block = kernel_hal::drivers::all_block().find("nvme").unwrap();
    
    
    use linux_object::time::*;
    let time_old = TimeSpec::now().sec;

    info!("sleep 5");
    while (TimeSpec::now().sec - time_old) < 5 {

    }


    
    
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

    info!("sleep 5");
    let time_old = TimeSpec::now().sec;
    while (TimeSpec::now().sec - time_old) < 5 {

    }


    // static mut read_buf:[u8; 512] = [0u8; 512];
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
    // unsafe{
    //     info!("read_buf = {:?}", read_buf);
    // }
   
    

    // static buf2:&[u8] = &[3u8;512];
    // unsafe{
    //     let f3 = nvme_block.async_write_block(2, &buf2);

    // }

    // // static mut read_buf:[u8; 512] = [0u8; 512];
    // unsafe{
    //     let f4 = nvme_block.async_read_block(2, &mut read_buf);
    // }
    
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





use lock::Mutex;

// //Task包装协程
// pub struct Task{
//     // future
//     pub future: Mutex<Pin<Box<dyn Future<Output=()> + Send>>>, 
// }

// impl Task{
//     pub fn new(future: Pin<Box<dyn Future<Output=()> +  Send >>) -> Self{
//         Task{
//             future: Mutex::new(future),
//         }
//     }
//     pub fn do_wake(self: &Arc<Self>) {
//         // todo!()
//     }
// }

// impl Future for Task {
//     type Output = usize;
//     fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
//         let mut f = self.future.lock();
//         match f.as_mut().poll(cx) {
//             Poll::Ready(_) => {
//                 Poll::Ready(1)
//             },
//             Poll::Pending => {
//                 Poll::Pending
//             }
//         }

//     }
// }

// use alloc::collections::VecDeque;
// pub struct TaskQueue {
//     pub queue: VecDeque<Arc<Task>>,
// }
// impl TaskQueue {
//     pub fn add_task(&mut self, task: Task) {
//         self.queue.push_front(Arc::new(task));
//     }

//     pub fn add_arc_task(&mut self, task: Arc<Task>) {
//         self.queue.push_back(task);
//     }

//     pub fn peek_task(&mut self) -> Option<Arc<Task>> {
//         self.queue.pop_front()
//     }

//     pub fn is_empty(&self) -> bool {
//         self.queue.is_empty()
//     }

// }

// impl woke::Woke for Task {
//     fn wake_by_ref(task: &Arc<Self>) {
//         task.do_wake()
//     }
// }


// lazy_static::lazy_static! {
//     pub static ref TASK_QUEUE: Arc<Mutex<Box<TaskQueue>>> =
//     Arc::new(
//         Mutex::new(
//             Box::new(
//                 TaskQueue {
//                     queue: VecDeque::new()
//                 }
//             )
//         )
//     );
// }

