#![allow(unused)]

use crate::{internal::{FPErr, FPResult}, FP_BIT_IST};

use super::{page_metas::{PageAddrTS, PageKVTS}, PageHeaderV2};

/**
 * In-page tuple header reference.
 */
#[derive(Debug, Clone, PartialEq)]
pub(super) struct CellReader<'a> {
    page_header: PageHeaderV2,
    start: &'a [u8],
    cur: &'a [u8],
}

impl<'a> CellReader<'a>  {

    pub(crate) fn new(bytes: &'a [u8], page_header: PageHeaderV2) -> CellReader<'a>{
        Self {
            start: bytes,
            cur: bytes,
            page_header
        }
    }

    fn cell_deacriptor(&self) -> CellDescriptor {
        CellDescriptor(self.cur[0])
    }

    fn read_cell(&self) {
        if self.cell_deacriptor().is_short_type() {
            self.read_short_cell()
        } else {
            self.read_normal_cell()
        }
    }

    fn read_normal_cell(&self) {

    }

    fn read_short_cell(&self) {
        
    }
}

impl Iterator for CellReader<'_> {
    type Item = Cell;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

#[derive(Debug, Clone, PartialEq)]
struct CellDescriptor(u8);

impl CellDescriptor {
    fn is_short_type(&self) -> bool {
        FP_BIT_IST!(self.0, Cell::SHORT_TYPE_MASK)
    }
}

pub(crate) enum Cell {
    KV(CellKV),
    Addr(CellAddr),
}

impl Cell {
    const SHORT_TYPE_MASK:u8 = 0x01;
}

struct CellDataIn {
    offset: usize,       /* Offset to the starting position of cell in page.  */
    cell: &'static [u8],
    data: &'static[u8],
    prefix: Option<&'static[u8]>,
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
    mvcc_meta: PageKVTS,
}

pub(crate) struct CellAddr {
    data: CellData,
    mvcc_meta: PageAddrTS,
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
