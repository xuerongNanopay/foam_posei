#![allow(unused)]

use std::{ops::{Deref, Range}, sync::Arc};

use crate::internal::FPResult;

use super::{TupleReader, PageHeaderRaw, PageHeaderV2};

pub(super) struct DiskPage {
    header_offset: usize,
    cell_offset: usize,
    header: PageHeaderV2,
    disk: DiskSlice,
}

impl DiskPage {
    pub(super) fn new(disk_page: Vec<u8>, header_offset: usize, cell_offset: usize) -> FPResult<Self> {
        let (_, raw_page_header) = PageHeaderRaw::deserialize(&disk_page[..])?;

        let disk = DiskSlice::new(disk_page);
        let header = raw_page_header.get_from_raw();

        Ok(Self {
            header_offset,
            cell_offset,
            header,
            disk,
        })
    }

    pub(super) fn cell_reader(&self) -> TupleReader {
        TupleReader::new(self.disk.slice(self.cell_offset..self.disk.len()), self.header)
    }

    pub(super) fn header(&self) -> PageHeaderV2 {
        self.header
    }
}

#[derive(Clone)]
pub(super) struct DiskSlice {
    data: &'static[u8],
    disk: Arc<Vec<u8>>
}

impl DiskSlice {
    pub(super) fn new(
        disk_page: Vec<u8>
    ) -> Self {
        let disk = Arc::new(disk_page);
        let bytes:&[u8] = disk.deref();
        /* Suppress compile check. */
        let data = unsafe { &*(bytes as *const [u8]) };

        Self {
            disk,
            data,
        }
    }

    #[inline]
    #[must_use]
    pub(super) fn slice(&self, range: Range<usize>) -> Self {
        Self {
            disk: Arc::clone(&self.disk),
            data: &self.data[range],
        }
    }

    #[inline]
    #[must_use]
    pub(super) fn reset(&self) -> Self {
        let disk = Arc::clone(&self.disk);
        let bytes:&[u8] = disk.deref();
        let data = unsafe { &*(bytes as *const [u8]) };
        Self {
            disk,
            data,
        }
    }

    #[inline]
    pub(super) fn len(&self) -> usize {
        self.data.len()
    }

    pub(super) fn to_vec(&self) -> Vec<u8> {
        self.data.to_vec()
    }
}

impl Deref for DiskSlice {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_sync<T: Sync>() {}

    #[test]
    fn test_page() {
        is_sync::<DiskPage>();
    }
}
