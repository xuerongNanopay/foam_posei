#![allow(unused)]

use crate::{FP_BIT_REVERSE_32, FP_BIT_REVERSE_64, FP_REINTERPRET_CAST_BUF};

pub(super) struct PageHeader {

}

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
    pub(crate) fn cast_into(raw_data: &[u8]) -> &'static PageHeaderRaw {
        FP_REINTERPRET_CAST_BUF!(raw_data, PageHeaderRaw)
    }

    fn get_from_raw(&self) -> PageHeaderRaw {
        Self {
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
            size:  if cfg!(target_endian = "big") { 
                FP_BIT_REVERSE_32!(self.size)
            } else {
                self.size
            },
            entries:  if cfg!(target_endian = "big") { 
                FP_BIT_REVERSE_32!(self.entries)
            } else {
                self.entries
            },
            r#type: self.r#type,
            flag: self.flag,
            unused: self.unused,
            version: self.version,
        }
    }

    fn set_to_raw(&mut self, page_header: PageHeaderRaw) {
        if cfg!(target_endian = "big") { 
            self.column_number = FP_BIT_REVERSE_64!(page_header.column_number);
            self.write_epoch = FP_BIT_REVERSE_64!(page_header.write_epoch);
            self.size = FP_BIT_REVERSE_32!(page_header.size);
            self.entries = FP_BIT_REVERSE_32!(page_header.entries);
        }
        self.r#type = page_header.r#type;
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
}
