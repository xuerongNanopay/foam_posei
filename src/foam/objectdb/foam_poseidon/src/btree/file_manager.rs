#![allow(unused)]

use crate::{disk::DiskManager, error::FP_NO_ERR, internal::FPResult};

struct FileManager {

}

impl FileManager {
    
}

impl DiskManager for FileManager {
    fn read(&self, addr: &[u8]) -> FPResult<Vec<u8>> {
        Err(FP_NO_ERR)
    }
}