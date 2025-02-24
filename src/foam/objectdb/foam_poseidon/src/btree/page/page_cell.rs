#![allow(unused)]

use crate::{btree::buf, internal::{FPErr, FPResult}, FP_BIT_IST, FP_BIT_MSK};

use super::{page_metas::{PageAddrTS, PageKVTS}, PageHeaderV2};

/**
 * In-page tuple header reference.
 */
#[derive(Debug, Clone, PartialEq)]
pub(super) struct CellReader<'a> {
    page_header: PageHeaderV2,
    start: &'a [u8],
    cur: &'a [u8],
    read_cells: u32,
    offset: usize,
}

impl<'a> CellReader<'a>  {

    pub(crate) fn new(buffer: &'a [u8], page_header: PageHeaderV2) -> CellReader<'a>{
        Self {
            start: buffer,
            cur: buffer,
            page_header,
            read_cells: 0,
            offset: 0,
        }
    }
}

impl Iterator for CellReader<'_> {
    type Item = Cell;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur.len() == 0 || self.read_cells == self.page_header.cells_or_flowlen {
            return None;
        }

        let descriptor = CellDescriptor(self.cur[0]);

        let cell_type = descriptor.get_type();

        if descriptor.is_short_type() {
            match cell_type {
                CellType::Short(c) => {
                    match c.0 {
                        Cell::SHORT_KEY_PFX => {
                            let prefix = self.cur[1];
                            let size = (descriptor.0 >> Cell::SHORT_SHIFT) as usize;
                            /* skip lifetime check. */
                            let data =  unsafe { &*(&self.cur[2..] as *const [u8]) };
                            let cell =  unsafe { &*(&self.cur[..2+size] as *const [u8]) };
                            return Some(Cell::KV(CellKV{
                                data: CellData::InPage(CellDataIn{
                                    cell,
                                    data,
                                    prefix: Some(prefix),
                                })
                            }))
                        },
                        Cell::SHORT_KEY | Cell::SHORT_VALUE => {
                            let prefix = 0u8;
                            let size = (descriptor.0 >> Cell::SHORT_SHIFT) as usize;
                            let data =  unsafe { &*(&self.cur[1..] as *const [u8]) };
                            let cell =  unsafe { &*(&self.cur[..1+size] as *const [u8]) };
                            return Some(Cell::KV(CellKV{
                                data: CellData::InPage(CellDataIn{
                                    cell,
                                    data,
                                    prefix: None,
                                })
                            }))
                        },
                        _ => {
                            panic!("impossible code with cell type: {}", c.0);
                        }
                    };
                },
                _ => {
                    panic!("impossible code with cell type: {}", descriptor.0);
                }
            };
        }

        None
    }
}

#[derive(Debug, Clone, PartialEq)]
struct CellDescriptor(u8);

impl CellDescriptor {
    #[inline]
    fn is_short_type(&self) -> bool {
        FP_BIT_IST!(self.0, Cell::SHORT_TYPE_MASK)
    }

    #[inline]
    fn get_type(&self) -> CellType {
        let mut t = self.0;
        if self.is_short_type() {
            FP_BIT_MSK!(t, Cell::SHORT_TYPE_MASK);
            CellType::Short(CellTypeShort(t))
        } else {
            FP_BIT_MSK!(t, Cell::LONG_TYPE_MASK);
            CellType::Long(CellTypeLong(t))
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum CellType {
    Short(CellTypeShort),
    Long(CellTypeLong),
}

#[derive(Debug, Clone, PartialEq)]
struct CellTypeShort(u8);

#[derive(Debug, Clone, PartialEq)]
struct CellTypeLong(u8);

pub(crate) enum Cell {
    KV(CellKV),
    Addr(CellAddr),
}

impl Cell {
    const SHORT_TYPE_MASK:u8 = 0x03;
    const LONG_TYPE_MASK:u8  = 0xf0;

    const SHORT_MAX_LEN:usize   = 63;
    const SHORT_SHIFT:u8        = 2;

    const SHORT_KEY:     u8 = 0x01;
    const SHORT_KEY_PFX: u8 = 0x02;
    const SHORT_VALUE:   u8 = 0x03;

    const ADDR_DEL:      u8 = 0;
    const ADDR_INTERNAL: u8 = 1 << 4;
    const ADDR_LEAF:     u8 = 2 << 4;
    const ADDR_LEAF_NO:  u8 = 3 << 4;

    const KV_DEL:        u8 = 4 << 4;
    const KEY:           u8 = 5 << 4;
    const KEY_OVFL:      u8 = 6 << 4;
    const KEY_PFX:       u8 = 7 << 4;
    const VALUE:         u8 = 8 << 4;
    const VALUE_OVFL:    u8 = 9 << 4;
    const VALUE_COPY:    u8 = 10 << 4;
    const KEY_OVFL_DEL:  u8 = 11 << 4;
    const VALUE_OVFL_DEL:u8 = 12 << 4;
    
}

struct CellDataIn {
    cell: &'static [u8],
    data: &'static[u8],
    prefix: Option<u8>,
}

struct CellDataOff {
    data: Vec<u8>
}

enum CellData {
    InPage(CellDataIn),
    OffPage(CellDataOff),
}

/* Implement TryFrom */
pub(crate) struct CellKV {
    data: CellData,
    // mvcc_meta: PageKVTS,
}

pub(crate) struct CellAddr {
    data: CellData,
    // mvcc_meta: PageAddrTS,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_reader() {
        // let buffer = [0u8, 10];
        // let reader = CellReader::try_from(&buffer[..]);
    }
}
