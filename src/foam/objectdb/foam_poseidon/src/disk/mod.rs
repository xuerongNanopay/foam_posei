#![allow(unused)]

use crate::internal::FPResult;

pub(super) trait DiskManager {
    fn read(&self, addr: &[u8]) -> FPResult<Vec<u8>>;
}