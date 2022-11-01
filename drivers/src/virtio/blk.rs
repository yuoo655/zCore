use lock::Mutex;
use virtio_drivers::{VirtIOBlk as InnerDriver, VirtIOHeader};

use crate::scheme::{BlockScheme, Scheme, block};
use crate::DeviceResult;

pub struct VirtIoBlk<'a> {
    inner: Mutex<InnerDriver<'a>>,
}

impl<'a> VirtIoBlk<'a> {
    pub fn new(header: &'static mut VirtIOHeader) -> DeviceResult<Self> {
        Ok(Self {
            inner: Mutex::new(InnerDriver::new(header)?),
        })
    }
}

impl<'a> Scheme for VirtIoBlk<'a> {
    fn name(&self) -> &str {
        "virtio-blk"
    }

    fn handle_irq(&self, _irq_num: usize) {
        self.inner.lock().ack_interrupt();
    }
}

use async_trait::async_trait;
use alloc::boxed::Box;
#[async_trait]
impl<'a> BlockScheme for VirtIoBlk<'a> {
    fn read_block(&self, block_id: usize, buf: &mut [u8]) -> DeviceResult {
        self.inner.lock().read_block(block_id, buf)?;
        Ok(())
    }

    fn write_block(&self, block_id: usize, buf: &[u8]) -> DeviceResult {
        self.inner.lock().write_block(block_id, buf)?;
        Ok(())
    }

    fn flush(&self) -> DeviceResult {
        Ok(())
    }

    async fn async_read_block(&self, block_id: usize, buf: &mut [u8]){
        warn!("111");
    }

    async fn async_write_block(&self, block_id: usize, buf: &[u8]){
        warn!("111");
    }
}
