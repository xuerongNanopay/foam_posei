#![allow(unused)]

use crate::{error::FP_BTREE_PAGE_ILL_HEADER_LEN, internal::FPResult, FP_ASSERT, FP_BIT_REVERSE_32, FP_BIT_REVERSE_64, FP_REINTERPRET_CAST_BUF, FP_SIZE_OF};

use super::PageTypeV2;


#[derive(Debug, Clone, PartialEq)]
pub(super) struct PageHeader {
    column_number: u64,
    write_epoch: u64,
    in_memory_size: u32,
    entries: u32,
    r#type: PageTypeV2,
    flag: u8,
    unused: u8,
    version: u8,
}

impl PageHeader {
    const FLAG_COMPRESSED: u8 = 0x01;
    const FLAG_ENCRYPTED:  u8 = 0x02;
    const FLAG_UNUSED:     u8 = 0x03;
}

/**
 * In-page page header reference.
 */
#[derive(Debug, Clone, PartialEq)]
#[repr(packed)]
pub(super) struct PageHeaderRaw {
    column_number: u64,
    write_epoch: u64,
    size: u32,
    entries: u32,
    r#type: u8,
    flag: u8,
    unused: u8,
    version: u8,
}

impl PageHeaderRaw {
    #[must_use]
    #[inline(always)]
    pub(crate) fn deserialize(raw_data: &[u8]) -> FPResult<(usize, &'static PageHeaderRaw)> {
        FP_ASSERT!(raw_data.len() >= FP_SIZE_OF!(PageHeaderRaw), FP_BTREE_PAGE_ILL_HEADER_LEN);

        Ok((FP_SIZE_OF!(PageHeaderRaw), FP_REINTERPRET_CAST_BUF!(raw_data, PageHeaderRaw)))
    }

    pub(crate) fn get_from_raw(&self) -> PageHeader {
        PageHeader {
            column_number:  if cfg!(target_endian = "big") { 
                FP_BIT_REVERSE_64!(self.column_number)
            } else {
                self.column_number
            },
            write_epoch:  if cfg!(target_endian = "big") { 
                FP_BIT_REVERSE_64!(self.write_epoch)
            } else {
                self.write_epoch
            },
            in_memory_size:  if cfg!(target_endian = "big") { 
                FP_BIT_REVERSE_32!(self.size)
            } else {
                self.size
            },
            entries:  if cfg!(target_endian = "big") { 
                FP_BIT_REVERSE_32!(self.entries)
            } else {
                self.entries
            },
            r#type: PageTypeV2::try_from_code(self.r#type).unwrap(),
            flag: self.flag,
            unused: self.unused,
            version: self.version,
        }
    }

    pub(crate) fn set_to_raw(&mut self, page_header: PageHeader) {
        if cfg!(target_endian = "big") { 
            self.column_number = FP_BIT_REVERSE_64!(page_header.column_number);
            self.write_epoch = FP_BIT_REVERSE_64!(page_header.write_epoch);
            self.size = FP_BIT_REVERSE_32!(page_header.in_memory_size);
            self.entries = FP_BIT_REVERSE_32!(page_header.entries);
        }
        self.r#type = page_header.r#type.to_code();
        self.flag = page_header.flag;
        self.unused = page_header.unused;
        self.version = page_header.version;
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FP_ALIGN_OF, FP_SIZE_OF};

    const RAW_PAGE_HEADER_SISE: usize = 28;

    #[test]
    fn test_page_header_size() {
        assert_eq!(RAW_PAGE_HEADER_SISE, FP_SIZE_OF!(PageHeaderRaw))
    }

    #[test]
    fn test_set_page_header() {
        let raw_data = vec![0u8; 28];
        let (_, header) = PageHeaderRaw::deserialize(&raw_data[..]).unwrap();
        
    }
}
