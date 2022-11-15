use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
// use core::mem::size_of;
use alloc::sync::Arc;
// use async_std::fs::write;
use alloc::boxed::Box;

use core::ptr::{read_volatile, write_volatile};

use crate::scheme::{BlockScheme, Scheme};
use crate::DeviceResult;

use lock::Mutex;
use lock::MutexGuard;

use super::nvme_queue::*;
use super::nvme_defs::*;

pub struct NvmeRes(Arc<Mutex<NvmeQueue<ProviderImpl>>>);

pub struct NvmeInterface {
    name: String,

    admin_queue: Arc<Mutex<NvmeQueue<ProviderImpl>>>,

    io_queues: Vec<Arc<Mutex<NvmeQueue<ProviderImpl>>>>,

    bar: usize,

    irq: usize,
}

impl NvmeInterface {
    pub fn new(bar: usize, irq: usize) -> DeviceResult<NvmeInterface> {
        let admin_queue = Arc::new(Mutex::new(NvmeQueue::new(0, 0)));

        let io_queues = vec![Arc::new(Mutex::new(NvmeQueue::<ProviderImpl>::new(1, 0x8)))];

        let mut interface = NvmeInterface {
            name: String::from("nvme"),
            admin_queue,
            io_queues,
            bar,
            irq,
        };

        interface.init();

        Ok(interface)
    }

    // config admin queue ,io queue
    pub fn init(&mut self) {

        self.nvme_configure_admin_queue();

        self.nvme_alloc_io_queue();
    }

    pub fn get_name_irq(&self) -> (String, usize) {
        (self.name.clone(), self.irq)
    }
}

impl NvmeInterface {

    pub fn nvme_queue_write_sq_db(&self, nvmeq: NvmeQueue<ProviderImpl>, write_sq: bool) {
        let mut nvmeq = nvmeq;
	    if !write_sq {
            let mut next_tail = nvmeq.sq_tail + 1;

	    	if next_tail == nvmeq.q_depth{
	    		next_tail = 0;
            }
	    	if next_tail != nvmeq.last_sq_tail{
	    		return;
            }
	    }
        let db = self.bar + NVME_REG_DBS + nvmeq.db_offset;
        unsafe { write_volatile(db as *mut u32, nvmeq.sq_tail as u32) }
        nvmeq.last_sq_tail = nvmeq.sq_tail;
    }

    pub fn nvme_submit_command(&self, nvmeq: NvmeQueue<ProviderImpl>, cmd: NvmeCommonCommand){
        let mut nvmeq = nvmeq;
        let sq_tail = nvmeq.sq_tail;
        nvmeq.sq[sq_tail].write(cmd);
        self.nvme_queue_write_sq_db(nvmeq, true);
    }

    pub fn submit_sync_command(&mut self, cmd: NvmeCommonCommand){        
        let mut admin_queue = self.admin_queue.lock();

        let dbs = self.bar + NVME_REG_DBS;

        let sq_tail = admin_queue.sq_tail;
        let cq_head = admin_queue.cq_head;

        admin_queue.sq[sq_tail].write(cmd);
        admin_queue.sq_tail += 1;

        let admin_q_db = dbs + admin_queue.db_offset;
        unsafe { write_volatile(admin_q_db as *mut u32, (sq_tail+1) as u32) }

        loop {
            let status = admin_queue.cq[cq_head].read();
            if status.status != 0 {
                unsafe { write_volatile((admin_q_db + 0x4) as *mut u32, (cq_head+1) as u32) }
                admin_queue.cq_head += 1;
                break;
            }
        }
    }

    pub fn nvme_configure_admin_queue(&mut self) {
        let admin_queue = self.admin_queue.lock();

        let bar = self.bar;
        let dbs = bar + NVME_REG_DBS;

        let sq_dma_pa = admin_queue.sq_pa as u32;
        let cq_dma_pa = admin_queue.cq_pa as u32;
        let data_dma_pa = admin_queue.data_pa as u64;

        drop(admin_queue);

        let aqa_low_16 = 31_u16;
        let aqa_high_16 = 31_u16;
        let aqa = (aqa_high_16 as u32) << 16 | aqa_low_16 as u32;
        let aqa_address = bar + NVME_REG_AQA;

        // 将admin queue配置信息写入nvme设备寄存器AQA (admin_queue_attributes)
        unsafe {
            write_volatile(aqa_address as *mut u32, aqa);
        }

        // 将admin queue的sq dma物理地址写入nvme设备上的寄存器ASQ
        let asq_address = bar + NVME_REG_ASQ;
        unsafe {
            write_volatile(asq_address as *mut u32, sq_dma_pa);
        }

        // 将admin queue的cq dma物理地址写入nvme设备上的寄存器ACQ
        let acq_address = bar + NVME_REG_ACQ;
        unsafe {
            write_volatile(acq_address as *mut u32, cq_dma_pa);
        }

        // enable ctrl
        let mut ctrl_config = NVME_CC_ENABLE | NVME_CC_CSS_NVM;
        ctrl_config |= 0 << NVME_CC_MPS_SHIFT;
        ctrl_config |= NVME_CC_ARB_RR | NVME_CC_SHN_NONE;
        ctrl_config |= NVME_CC_IOSQES | NVME_CC_IOCQES;

        unsafe { write_volatile((bar + NVME_REG_CC) as *mut u32, ctrl_config) }

        let _dev_status = unsafe { read_volatile((bar + NVME_REG_CSTS) as *mut u32) };

        // warn!("nvme status {}", _dev_status);

        // config identify
        let mut cmd = NvmeIdentify::new();
        cmd.prp1 = data_dma_pa;
        cmd.command_id = 0x1018; //random number
        cmd.nsid = 1;
        let common_cmd = unsafe { core::mem::transmute(cmd) };
        self.submit_sync_command(common_cmd);

    }

    pub fn nvme_alloc_io_queue(&mut self) {

        let bar = self.bar;
        let sq_dma_pa = self.io_queues[0].as_ref().lock().sq_pa as u32;
        let cq_dma_pa = self.io_queues[0].as_ref().lock().cq_pa as u32;
        let data_dma_pa = self.io_queues[0].as_ref().lock().data_pa as u64;

        let dev_dbs = bar + NVME_REG_DBS;
        let admin_q_db = dev_dbs;


        //nvme_set_queue_count
        let mut cmd = NvmeCommonCommand::new();
        cmd.opcode = 0x09;
        cmd.command_id = 0x101a;
        cmd.nsid = 1;
        cmd.cdw10 = 0x7;
        let common_cmd = unsafe {
            core::mem::transmute(cmd)
        };
        self.submit_sync_command(common_cmd);


        //nvme create cq
        let mut cmd = NvmeCommonCommand::new();
        cmd.opcode = 0x05;
        cmd.command_id = 0x101b;
        cmd.nsid = 1;
        cmd.prp1 = cq_dma_pa as u64;
        cmd.cdw10 = 0x3ff0001;
        cmd.cdw11 = 0x3;
        // cmd.flags = 3;
        let common_cmd = unsafe {
            core::mem::transmute(cmd)
        };
        self.submit_sync_command(common_cmd);


        //nvme create sq
        let mut cmd = NvmeCommonCommand::new();
        cmd.opcode = 0x01;
        cmd.command_id = 0x2018;
        cmd.nsid = 1;
        cmd.prp1 = sq_dma_pa as u64;
        cmd.cdw10 = 0x3ff0001;
        cmd.cdw11 = 0x10001;

        let common_cmd = unsafe {
            core::mem::transmute(cmd)
        };
        self.submit_sync_command(common_cmd);
    }


    fn send_read_command(&self, block_id:usize, read_buf: &mut [u8], cid: usize)-> usize{
        // 这里dma addr 就是buffer的地址
        let ptr = read_buf.as_mut_ptr();
        let addr = virt_to_phys(ptr as usize);

        // build nvme read command
        let mut cmd = NvmeRWCommand::new_read_command();
        cmd.nsid = 1;
        cmd.prp1 = addr as u64;
        cmd.command_id = cid as u16;
        cmd.length = 0;
        cmd.slba = block_id as u64;

        //transfer to common command
        let common_cmd = unsafe { core::mem::transmute(cmd) };

        let mut io_queue = self.io_queues[0].lock();
        self.sync_command( &mut io_queue, common_cmd);
        
        cid as usize
    }
    fn send_write_command(&self, block_id:usize, write_buf: &[u8], cid: usize) -> usize{
        warn!("write block");
        let ptr = write_buf.as_ptr();
        let addr = virt_to_phys(ptr as usize);
        // build nvme write command
        let mut cmd = NvmeRWCommand::new_write_command();
        cmd.nsid = 1;
        cmd.prp1 = addr as u64;
        cmd.length = 0;
        cmd.command_id = cid as u16;
        cmd.slba = block_id as u64;
        // transmute to common command
        let common_cmd = unsafe { core::mem::transmute(cmd) };

        let mut io_queue = self.io_queues[0].lock();
        self.sync_command( &mut io_queue, common_cmd);
    
        cid as usize
    }

}


use async_trait::async_trait;

#[async_trait]
impl BlockScheme for NvmeInterface {
    // 每个NVMe命令中有两个域：PRP1和PRP2，Host就是通过这两个域告诉SSD数据在内存中的位置或者数据需要写入的地址
    // 首先对prp1进行读写，如果数据还没完，就看数据量是不是在一个page内，在的话，只需要读写prp2内存地址就可以了，数据量大于1个page，就需要读出prp list

    // 由于只读一块, 小于一页, 所以只需要prp1
    // prp1 = dma_addr
    // prp2 = 0

    // prp设置
    // uboot中对应实现 nvme_setup_prps
    // linux中对应实现 nvme_pci_setup_prps

    // SLBA = start logical block address
    // length = 1 = 512B
    // 1 SLBA = 512B
    fn read_block(&self, block_id: usize, read_buf: &mut [u8]) -> DeviceResult {
        // let io_queue = self.io_queues[0].lock();
        let db_offset = 0x8;

        let bar = self.bar;

        let dbs = bar + NVME_REG_DBS;
        // let db_offset = 0x8;

        // 这里dma addr 就是buffer的地址
        let ptr = read_buf.as_mut_ptr();
        let addr = virt_to_phys(ptr as usize);

        // build nvme read command
        let mut cmd = NvmeRWCommand::new_read_command();
        cmd.nsid = 1;
        cmd.prp1 = addr as u64;
        cmd.command_id = 101;
        cmd.length = 0;
        cmd.slba = block_id as u64;

        //transfer to common command
        let common_cmd = unsafe { core::mem::transmute(cmd) };

        let mut io_queue = self.io_queues[0].lock();
        self.sync_command( &mut io_queue, common_cmd);
        self.nvme_poll_cq(&mut io_queue);

        Ok(())
    }

    // prp1 = write_buf physical address
    // prp2 = 0
    // SLBA = start logical block address
    // length = 0 = 512B
    fn write_block(&self, block_id: usize, write_buf: &[u8]) -> DeviceResult {
        let db_offset = 0x8;
        let ptr = write_buf.as_ptr();

        let addr = virt_to_phys(ptr as usize);

        // build nvme write command
        let mut cmd = NvmeRWCommand::new_write_command();
        cmd.nsid = 1;
        cmd.prp1 = addr as u64;
        cmd.length = 0;
        cmd.command_id = 100;
        cmd.slba = block_id as u64;

        // transmute to common command
        let common_cmd = unsafe { core::mem::transmute(cmd) };

        let mut io_queue = self.io_queues[0].lock();
        self.sync_command( &mut io_queue, common_cmd);
        self.nvme_poll_cq(&mut io_queue);

        Ok(())
    }

    fn flush(&self) -> DeviceResult {
        Ok(())
    }


    async fn async_read_block(&self, block_id: usize, read_buf: &mut [u8]){
        warn!("async read block");
        let cid = NVME_COMMAND_ID.lock().load(Ordering::SeqCst);
        NVME_COMMAND_ID.lock().store(cid + 1, Ordering::Relaxed);

        self.send_read_command(block_id, read_buf, cid);
        let f =  NvmeFuture::new(cid);
        f.await;
    }

    async fn async_write_block(&self, block_id: usize, write_buf: &[u8]){
        warn!("async write block");
        let cid = NVME_COMMAND_ID.lock().load(Ordering::SeqCst);
        NVME_COMMAND_ID.lock().store(cid + 1, Ordering::Relaxed);

        self.send_write_command(block_id, write_buf, cid);
        let f =  NvmeFuture::new(cid);
        f.await;
    }
}

pub struct NvmeCommonId(usize);

use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll, Waker},
};
use core::sync::atomic::{AtomicBool, Ordering, AtomicUsize};
use alloc::{collections::BTreeMap};


lazy_static::lazy_static! {
    pub static ref NVME_MAP: Mutex<BTreeMap<usize, (Waker, Arc<AtomicBool>)>> = Mutex::new(BTreeMap::new());
}


lazy_static::lazy_static! {
    pub static ref NVME_COMMAND_ID: Mutex<AtomicUsize> = Mutex::new(AtomicUsize::new(100));
}


pub struct NvmeFuture{
    command_id: usize,
    irq_occurred: Arc<AtomicBool>,
    
}

impl NvmeFuture{
    fn new(id: usize)-> Self{

        Self {
            command_id: id,
            irq_occurred: Arc::new(AtomicBool::new(false))
        }
    }
}

impl Future for NvmeFuture{

    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        // info!("poll nvme future");
        let waker = cx.waker().clone();

        if !self.irq_occurred.load(Ordering::SeqCst) {
            NVME_MAP.lock().insert(self.command_id, (waker, self.irq_occurred.clone()),);
        }else{
            return Poll::Ready(());
        }
        Poll::Pending
    }
}



impl Scheme for NvmeInterface {
    fn name(&self) -> &str {
        "nvme"
    }

    fn handle_irq(&self, irq: usize) {
        warn!("nvme device irq {}", irq);
        let mut io_queue = self.io_queues[0].lock();

        if self.nvme_cqe_pending(&mut io_queue){
            let head = io_queue.cq_head;
            let status = io_queue.cq[head].read();
            let id = status.command_id as usize;
            let nvmemap = NVME_MAP.lock();            
            if let Some((wake, abool)) = nvmemap.get(&id){
                abool.store(true, Ordering::Relaxed);    
                wake.wake_by_ref();
            }
            self.nvme_update_cq_head(&mut io_queue);
            self.nvme_ring_cq_doorbell(&mut io_queue);
        }
    }
}



impl NvmeInterface{
    // write command to submission queue and write sq doorbell to notify nvme device
    pub fn sync_command(&self, nvmeq: &mut MutexGuard<NvmeQueue<ProviderImpl>>, cmd: NvmeCommonCommand){
        let sq_tail = nvmeq.sq_tail;
        nvmeq.sq[sq_tail].write(cmd);
        
        if (nvmeq.sq_tail + 1) == nvmeq.q_depth{
            nvmeq.sq_tail = 0;
        }else{
            nvmeq.sq_tail += 1;
        }

        self.nvme_write_sq_db(nvmeq, true);
    }

    // check completion queue and update cq head cq doorbell until there is no pending command
    pub fn nvme_poll_cq(&self, nvmeq: &mut MutexGuard<NvmeQueue<ProviderImpl>>){

        while self.nvme_cqe_pending(nvmeq){
            self.nvme_update_cq_head(nvmeq);
            self.nvme_ring_cq_doorbell(nvmeq);
        }
    }

    // check if there is completed command in completion queue
    pub fn nvme_cqe_pending(&self, nvmeq:&mut  MutexGuard<NvmeQueue<ProviderImpl>>)-> bool{

        let cq_head = nvmeq.cq_head;
        let cqe = nvmeq.cq[cq_head].read();
        if (cqe.status & 1) == (nvmeq.cq_phase as u16) {
            return true;
        }else{
            return false;
        }
    }

    // notify nvme device we've completed the command
    pub fn nvme_ring_cq_doorbell(&self, nvmeq: &mut MutexGuard<NvmeQueue<ProviderImpl>>){
        let cq_head = nvmeq.cq_head;
        let q_db = self.bar + NVME_REG_DBS + nvmeq.db_offset;
        unsafe { write_volatile((q_db + 0x4) as *mut u32, cq_head as u32) }
    }

    // write submission queue doorbell to notify nvme device
    pub fn nvme_write_sq_db(&self, nvmeq: &mut MutexGuard<NvmeQueue<ProviderImpl>>, write_sq: bool) {
    
	    if !write_sq {
            let mut next_tail = nvmeq.sq_tail + 1;
	    	if next_tail == nvmeq.q_depth{
	    		next_tail = 0;
            }
	    	if next_tail != nvmeq.last_sq_tail{
	    		return;
            }
	    }

        let db = self.bar + NVME_REG_DBS + nvmeq.db_offset;
        unsafe { write_volatile(db as *mut u32, nvmeq.sq_tail as u32) }
        nvmeq.last_sq_tail = nvmeq.sq_tail;
    }

    // update completion queue head
    pub fn nvme_update_cq_head(&self, nvmeq: &mut MutexGuard<NvmeQueue<ProviderImpl>>) {
        let next_head = nvmeq.cq_head + 1;
        if next_head == nvmeq.q_depth{
            nvmeq.cq_head = 0;
            nvmeq.cq_phase ^= 1;
        }else{
            nvmeq.cq_head = next_head;
        }
    }
}


// // ...

// GetWakerFuture;

// target = Waker

// fn poll(self, cx) {
//     Poll::Ready(cx.waker().clone()))
// }

// async {

//     ccc.await
//     bb().await

// }

// bb(0 {


//     .await
// })