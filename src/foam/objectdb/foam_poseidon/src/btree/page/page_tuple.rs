#![allow(unused)]

use crate::{btree::buf, internal::{FPErr, FPResult}, util::compaction::varint, FP_BIT_IST, FP_BIT_MSK, FP_BIT_REVERSE_32, FP_REINTERPRET_CAST_BUF};

use super::{page_metas::{PageAddrTS, PageKVTS}, DiskSlice, PageHeaderV2};

/**
 * In-page tuple header reference.
 */
#[derive(Clone)]
pub(super) struct TupleReader {
    page_header: PageHeaderV2,
    cur: usize,
    disk_cells: DiskSlice,
    read_cells: u32,
}

impl<'a> TupleReader  {

    pub(crate) fn new(disk_cells: DiskSlice, page_header: PageHeaderV2) -> TupleReader{
        Self {
            cur: 0,
            disk_cells,
            page_header,
            read_cells: 0,
        }
    }
}

impl Iterator for TupleReader {
    type Item = Tuple;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur == self.disk_cells.len() || self.read_cells == self.page_header.cells_or_flowlen {
            return None;
        }

        let mut idx = self.cur;
        let bytes = &*self.disk_cells;
        let descriptor = TupleDescriptor(bytes[idx]);
        let raw_type = descriptor.get_raw_type();

        match raw_type {
            Tuple::SHORT_KEY_PFX => {
                let prefix = bytes[idx+1];
                let size = (descriptor.0 >> Tuple::SHORT_SHIFT) as usize;
                let data =  self.disk_cells.slice(idx+2..size);
                let cell =  self.disk_cells.slice(idx..2+size);

                self.cur += 2+size;
                self.read_cells += 1;
                return Some(Tuple::KV(KVTuple{
                    raw_type,
                    r#type: descriptor.get_collapse_type(),
                    is_overflow: false,
                    disk_tuple: DiskTuple{
                        cell,
                        data: Some(data),
                        prefix: Some(prefix),
                    }
                }))
            },
            Tuple::SHORT_KEY | Tuple::SHORT_VALUE => {
                let size = (descriptor.0 >> Tuple::SHORT_SHIFT) as usize;
                let data =  self.disk_cells.slice(idx+1..size);
                let cell =  self.disk_cells.slice(idx..1+size);

                self.cur += 1+size;
                self.read_cells += 1;
                return Some(Tuple::KV(KVTuple{
                    raw_type,
                    r#type: descriptor.get_collapse_type(),
                    is_overflow: false,
                    disk_tuple: DiskTuple{
                        cell,
                        data: Some(data),
                        prefix: None,
                    }
                }))
            },
            _ => {
               /* Normal Type */
            }
        };

        /* Normal type parsing. */
        let prefix = if raw_type == Tuple::KEY_PFX {
            let prefix = bytes[idx+1];
            idx += 2;
            Some(prefix)
        } else {
            idx += 1;
            None
        };

        //NEED TODO: parse mvcc fields.
        match raw_type {
            Tuple::ADDR_DEL | Tuple::ADDR_INTERNAL | Tuple::ADDR_LEAF | Tuple::ADDR_LEAF_NO => {

            },
            Tuple::KV_DEL | Tuple::VALUE | Tuple::VALUE_COPY | Tuple::VALUE_OVFL | Tuple::VALUE_OVFL_DEL => {

            }
            _ => {}
        };

        //NEED TODO: fast-truncate.
        //NEED TODO: column Run-Length Encoding.

        let ret = match raw_type {
            Tuple::VALUE_COPY => {
                //FEAT TODO: performace on disk space.
                //store previous no copy value.
                None
            },
            Tuple::KEY_OVFL | Tuple::KEY_OVFL_DEL | Tuple::VALUE_OVFL | Tuple::VALUE_OVFL_DEL |
            Tuple::ADDR_DEL | Tuple::ADDR_INTERNAL | Tuple::ADDR_LEAF | Tuple::ADDR_LEAF_NO |
            Tuple::KEY | Tuple::KEY_PFX | Tuple::VALUE => {
                let mut is_overflow = false;
                if matches!(raw_type, Tuple::KEY_OVFL | Tuple::KEY_OVFL_DEL | Tuple::VALUE_OVFL | Tuple::VALUE_OVFL_DEL)  {
                    is_overflow = true;
                }

                let (size, off) = varint::decode_uint(&bytes[idx..]).unwrap();
                let mut size = size as u32;
                if cfg!(target_endian = "big") { 
                    size = FP_BIT_REVERSE_32!(size);
                }
                let size = size as usize;

                idx += off;

                //FEAT TODO: reduce size field size on disk.

                let data =  self.disk_cells.slice(idx..idx+size);
                let cell =  self.disk_cells.slice(self.cur..idx+size);

                self.cur += idx+size;

                Some(Tuple::KV(KVTuple{
                    raw_type,
                    r#type: descriptor.get_collapse_type(),
                    is_overflow,
                    disk_tuple: DiskTuple{
                        cell,
                        data: Some(data),
                        prefix,
                    }
                }))
            },
            Tuple::KV_DEL => {
                let cell =  self.disk_cells.slice(self.cur..idx);

                self.cur += idx;

                Some(Tuple::KV(KVTuple{
                    raw_type,
                    r#type: descriptor.get_collapse_type(),
                    is_overflow: false,
                    disk_tuple: DiskTuple{
                        cell,
                        data: None,
                        prefix,
                    }
                }))
            },
            _ => {
                panic!("impossible code")
            },
        };

        self.read_cells += 1;

        ret
    }
}

#[derive(Debug, Clone, PartialEq)]
struct TupleDescriptor(u8);

impl TupleDescriptor {
    #[inline]
    fn is_short_type(&self) -> bool {
        FP_BIT_IST!(self.0, Tuple::SHORT_TYPE_MASK)
    }

    #[inline]
    fn get_raw_type(&self) -> u8 {
        let mut ret = self.0;
        if self.is_short_type() {
            FP_BIT_MSK!(ret, Tuple::SHORT_TYPE_MASK);
        } else {
            FP_BIT_MSK!(ret, Tuple::LONG_TYPE_MASK);
        }
        ret
    }

    #[inline]
    fn get_collapse_type(&self) -> u8 {
        let raw_type = self.get_raw_type();

        match raw_type {
            Tuple::SHORT_KEY | Tuple::SHORT_KEY_PFX | Tuple::KEY_PFX => {
                Tuple::KEY
            },
            Tuple::SHORT_VALUE => {
                Tuple::VALUE
            },
            Tuple::KEY_OVFL_DEL => {
                Tuple::KEY_OVFL
            },
            Tuple::VALUE_OVFL_DEL => {
                Tuple::VALUE_OVFL
            }
            _ => {raw_type}
        }
    }
}

pub(crate) enum Tuple {
    KV(KVTuple),
    Addr(AddrTuple),
}

impl Tuple {
    pub(super) const SHORT_TYPE_MASK:u8 = 0x03;
    pub(super) const LONG_TYPE_MASK:u8  = 0xf0;

    pub(super) const SHORT_MAX_LEN:usize   = 63;
    pub(super) const SHORT_SHIFT:u8        = 2;

    pub(super) const SHORT_KEY:     u8 = 0x01;
    pub(super) const SHORT_KEY_PFX: u8 = 0x02;
    pub(super) const SHORT_VALUE:   u8 = 0x03;

    pub(super) const ADDR_DEL:      u8 = 0;
    pub(super) const ADDR_INTERNAL: u8 = 1 << 4;
    pub(super) const ADDR_LEAF:     u8 = 2 << 4;
    pub(super) const ADDR_LEAF_NO:  u8 = 3 << 4;

    pub(super) const KV_DEL:        u8 = 4 << 4;
    pub(super) const KEY:           u8 = 5 << 4;
    pub(super) const KEY_OVFL:      u8 = 6 << 4;
    pub(super) const KEY_PFX:       u8 = 7 << 4;
    pub(super) const VALUE:         u8 = 8 << 4;
    pub(super) const VALUE_OVFL:    u8 = 9 << 4;
    pub(super) const VALUE_COPY:    u8 = 10 << 4;
    pub(super) const KEY_OVFL_DEL:  u8 = 11 << 4;
    pub(super) const VALUE_OVFL_DEL:u8 = 12 << 4;
    
}

struct DiskTuple {
    cell: DiskSlice,
    data: Option<DiskSlice>,
    prefix: Option<u8>,
}


/* Implement TryFrom */
pub(crate) struct KVTuple {
    disk_tuple: DiskTuple,
    is_overflow: bool,
    raw_type: u8,
    r#type: u8,
    // mvcc_meta: PageKVTS,
}

impl KVTuple {
    pub(super) fn raw_type(&self) -> u8 {
        self.raw_type
    }
    pub(super) fn is_overflow(&self) -> bool {
        self.is_overflow
    }
    pub(super) fn r#type(&self) -> u8 {
        self.r#type
    }

    pub(super) fn get_tuple_data(&self) -> Option<DiskSlice> {
        self.disk_tuple.data.clone()
    }
}


pub(crate) struct AddrTuple {
    data: DiskTuple,
    // mvcc_meta: PageAddrTS,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_reader() {
        // let buffer = [0u8, 10];
        // let reader = TupleReader::try_from(&buffer[..]);
    }
}
