#![allow(unused)]

use std::{ops::{Deref, Range}, sync::Arc};

use stable_deref_trait::StableDeref;

#[derive(Clone)]
pub(super) struct PageSlice {
    data: &'static[u8],
    disk: Arc<Vec<u8>>
}

impl PageSlice {
    pub(crate) fn new(
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
    pub(crate) fn slice(&self, range: Range<usize>) -> Self {
        Self {
            disk: Arc::clone(&self.disk),
            data: &self.data[range],
        }
    }

    #[inline]
    #[must_use]
    pub(crate) fn reset(&self) -> Self {
        let disk = Arc::clone(&self.disk);
        let bytes:&[u8] = disk.deref();
        let data = unsafe { &*(bytes as *const [u8]) };
        Self {
            disk,
            data,
        }
    }
}