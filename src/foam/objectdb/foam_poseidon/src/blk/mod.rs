#![allow(unused)]

use meta::BlkHeader;

use crate::{meta::FP_METAFILE};

pub mod handle;
mod pool;
mod mgr;
mod meta;
mod compress;
pub(crate) mod util;

static FP_BLOCK_INVALID_OFFSET: u64 = 0;

pub(crate) const FP_BLOCK_HEADER_LEN: usize = 30;

type RawBlk = Vec<u8>;

/**
 * Each .fp file has one FileHeader
 */
#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
struct FileHeader {
    magic: u32,
    major: u16,
    minor: u16,
    checksum: u32,
    reserved: u32,
}

impl FileHeader {
    fn maybe_convert_endian(&mut self) {
        if cfg!(target_endian = "big") { 
            self.magic = FP_BIT_REVERSE_32!(self.magic);
            self.major = FP_BIT_REVERSE_16!(self.major);
            self.minor = FP_BIT_REVERSE_16!(self.minor);
            self.checksum = FP_BIT_REVERSE_32!(self.checksum);
            self.reserved = FP_BIT_REVERSE_32!(self.reserved);
        }
    }
}

// #[repr(C)]
// #[derive(Debug, Default, Clone, Copy)]
// pub(crate) struct BlockRef {
//     source_id: u32,
//     offset: FPFileSize, /* offset in file */
//     size: FPFileSize, /* size of a block in file */
//     checksum: u32,
// }


#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct BlockHeader {
    disk_size: u32,
    checksum: u32,
    flags: u8,
    unused: [u8; 3],
}

impl BlockHeader {
    fn maybe_convert_endian(&mut self) {
        if cfg!(target_endian = "big") { 
            self.disk_size = FP_BIT_REVERSE_32!(self.disk_size);
            self.checksum = FP_BIT_REVERSE_32!(self.checksum);
        }
    }
}


pub(crate) struct BlkItem {
    pub(crate) mem: Vec<u8>,
    pub(crate) size: usize,
}

impl BlkItem {
    pub(crate) fn blk_header(&self) -> BlkHeader {
        let raw_blk_header = &self.mem[FP_SIZE_OF!(u8)..];
        BlkHeader::from(raw_blk_header)
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct BlockStat{
    allocation_size: u64,
    block_size: u64,
    block_magic: u64,
    block_major: u16,
    block_minor: u16,
}


fn is_internal_file(filename: &str) -> bool {
    if filename == FP_METAFILE {
        return true
    }
    false
}

#[cfg(test)]
mod tests {
    // use super::BlockHeader;

    use std::fs::File;
    use std::io::{self, Bytes, Read, Write};
    use std::{mem, vec};

    use crate::blk::FileHeader;

    #[test]
    fn write_block_header_to_file() {
        let filename = "/tmp/block_header.fp";
        let mut file = File::create(filename).unwrap();
        let mut w_buf = Vec::<u8>::with_capacity(FP_SIZE_OF!(FileHeader));
        unsafe { w_buf.set_len(FP_SIZE_OF!(FileHeader)); }
        w_buf.fill(0);
        let header = FP_REINTERPRET_CAST_BUF_MUT!(w_buf, FileHeader);

        header.checksum = 11;
        header.magic = 56;

        println!("{:?}", w_buf);
        println!("{:?}", header);
        file.write_all(&w_buf);
        file.sync_all();

        let mut file = File::open(filename).unwrap();
        let mut rbuf = vec![0u8; w_buf.len() as usize];
        file.read_exact(&mut rbuf);
        
        let b_header = FP_REINTERPRET_CAST_BUF!(rbuf, FileHeader);

        println!("{:?}", rbuf);
        println!("{:?}", b_header);

        let mut n_header = *b_header;
        println!("{:?}", n_header);
        n_header.checksum = 77;
        println!("{:?}", b_header);
        println!("{:?}", n_header);
    }
}