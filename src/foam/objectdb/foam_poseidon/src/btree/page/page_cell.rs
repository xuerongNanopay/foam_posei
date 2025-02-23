#![allow(unused)]

use crate::FP_BIT_IST;

/**
 * In-page tuple header reference.
 */
#[derive(Debug, Clone, PartialEq)]
struct CellReader(&'static [u8]);

impl CellReader {

    fn cell_deacriptor(&self) -> CellDescriptor {
        CellDescriptor(self.0[0])
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

impl Iterator for CellReader {
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

enum Cell {
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

struct CellKV {
    data: CellData,
}

struct CellAddr {
    data: CellData,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tuple_header_len() {

    }
}
