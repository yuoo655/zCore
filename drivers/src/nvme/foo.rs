use nvme_driver::NvmeInterface;
use nvme_driver::DmaAllocator;
use nvme_driver::IrqController;


use crate::scheme::{BlockScheme, Scheme};
use crate::DeviceResult;
use alloc::slice;
use core::marker::PhantomData;
use volatile::Volatile;

extern "C" {
    fn drivers_dma_alloc(pages: usize) -> PhysAddr;
    fn drivers_dma_dealloc(paddr: PhysAddr, pages: usize) -> i32;
    fn drivers_phys_to_virt(paddr: PhysAddr) -> VirtAddr;
    fn drivers_virt_to_phys(vaddr: VirtAddr) -> PhysAddr;
    fn drivers_timer_now_as_micros() -> u64;
}

pub const PAGE_SIZE: usize = 4096;

type VirtAddr = usize;
type PhysAddr = usize;



pub struct DmaProvider;

impl DmaAllocator for DmaProvider{

    fn dma_alloc(size: usize) -> usize{
        let paddr = unsafe { drivers_dma_alloc(size / PAGE_SIZE) };
        paddr
    }

    fn dma_dealloc(vaddr: usize, size: usize) -> usize {
        let paddr = unsafe { drivers_virt_to_phys(vaddr) as usize };
        unsafe { drivers_dma_dealloc(paddr, size / PAGE_SIZE) };
        0
    }

    fn phys_to_virt(phys: usize) -> usize {
        unsafe { drivers_phys_to_virt(phys) as usize }
    }

    fn virt_to_phys(virt: usize) -> usize{
        unsafe { drivers_virt_to_phys(virt) as usize }
    }
}


pub struct IrqProvider;

impl IrqController for IrqProvider{
    fn enable_irq(irq: usize){
    }

    fn disable_irq(irq: usize){   
    }
}



pub struct NvmeWrapper {
    nvme: NvmeInterface<DmaProvider, IrqProvider>,
}

impl NvmeWrapper{
    pub fn new(bar: usize, irq: usize) -> DeviceResult<NvmeWrapper> {
        let nvme = NvmeInterface::<DmaProvider, IrqProvider>::new(bar);

        let nvme_wrapper = NvmeWrapper{
            nvme: nvme,
        };
        Ok(nvme_wrapper)
    }
}


impl BlockScheme for NvmeWrapper{
    fn read_block(&self, block_id: usize, read_buf: &mut [u8]) -> DeviceResult {

        self.nvme.read_block(block_id, read_buf);
        Ok(())
    }

    fn write_block(&self, block_id: usize, write_buf: &[u8]) -> DeviceResult {
        self.nvme.write_block(block_id, write_buf);
        Ok(())
    }

    fn flush(&self) -> DeviceResult {
        Ok(())
    }

}


impl Scheme for NvmeWrapper {
    fn name(&self) -> &str {
        "nvme"
    }

    fn handle_irq(&self, irq: usize) {
        self.nvme.handle_irq()
    }
}