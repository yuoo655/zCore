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


    // use alloc::boxed::Box;
    // let irq = kernel_hal::drivers::all_irq().find("riscv-plic").unwrap();
    // let nvme = kernel_hal::drivers::all_block().find("nvme").unwrap();
    // let irq_num = 33;
    // let _r = irq.register_handler(irq_num, Box::new(move || nvme.handle_irq(irq_num)));

    // let _r = irq.unmask(irq_num);

    // let nvme_block = kernel_hal::drivers::all_block()
    // .find("nvme")
    // .unwrap();


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
