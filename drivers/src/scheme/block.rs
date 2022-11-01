use super::Scheme;
use crate::DeviceResult;
use async_trait::async_trait;

use alloc::boxed::Box;

#[async_trait]
pub trait BlockScheme: Scheme {
    fn read_block(&self, block_id: usize, buf: &mut [u8]) -> DeviceResult;
    fn write_block(&self, block_id: usize, buf: &[u8]) -> DeviceResult;
    fn flush(&self) -> DeviceResult;
    async fn async_read_block(&self, block_id: usize, buf: &mut [u8]);
    async fn async_write_block(&self, block_id: usize, buf: &[u8]);
}
