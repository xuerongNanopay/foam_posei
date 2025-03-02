#![allow(unused)]

use std::{ops::{Deref, Range}, sync::Arc};

use stable_deref_trait::StableDeref;

#[derive(Clone)]
pub(super) struct PageSlice {
    data: &'static[u8],
    disk: Arc<Vec<u8>>
}

impl PageSlice {
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
    pub(crate) fn slice(&self, range: Range<usize>) -> Self {
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

    pub(super) fn to_vec(&self) -> Vec<u8> {
        self.data.to_vec()
    }
}

impl Deref for PageSlice {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.data
    }
}