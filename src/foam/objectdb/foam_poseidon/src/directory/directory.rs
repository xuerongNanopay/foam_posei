#![allow(unused)]

use std::{path, sync::Arc};

use crate::{common::file::FileHandle, internal::FPResult, util::uri::Uri};

/**
 * Abstraction of disk manager.
 */
trait Directory: Send + Sync + 'static {
    fn file_handle(&self, path: Uri) -> FPResult<Arc<dyn FileHandle>>;

    // fn open_reader(&self, path: Uri) -> Result<>
}