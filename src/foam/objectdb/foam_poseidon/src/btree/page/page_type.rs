use std::fmt::Display;

use crate::{error::FP_ILLEGAL_ARGUMENT, internal::FPResult};

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub(super) enum PageType {
    #[default]
    Invalid     = 0,
    Meta        = 1,
    Overflow    = 2,
    ColLeafFix  = 3,
    ColLeafVar  = 4,
    ColInternal = 5,
    RowInternal = 6,
    RowLeaf     = 7,
}

impl Display for PageType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let short_str = match self {
            PageType::Invalid     => "PAGE_INVALID",
            PageType::Meta        => "PAGE_META",
            PageType::Overflow    => "PAGE_OVERFLOW",
            PageType::ColLeafFix  => "PAGE_COL_LEAF_FIX",
            PageType::ColLeafVar  => "PAGE_COL_LEAF_VAR",
            PageType::ColInternal => "PAGE_COL_INTERNAL",
            PageType::RowInternal => "PAGE_ROW_INTERNAL",
            PageType::RowLeaf     => "PAGE_ROW_LEAF",
        };
        write!(f, "{short_str}")
    }
}

impl PageType {
    pub(crate) fn to_code(self) -> u8 {
        self as u8
    }

    pub(crate) fn try_from_code(code: u8) -> FPResult<PageType> {
        match code {
            0 => Ok(PageType::Invalid),
            1 => Ok(PageType::Meta),
            2 => Ok(PageType::Overflow),
            3 => Ok(PageType::ColLeafFix),
            4 => Ok(PageType::ColLeafVar),
            5 => Ok(PageType::ColInternal),
            6 => Ok(PageType::RowInternal),
            7 => Ok(PageType::RowLeaf),
            _ => Err(FP_ILLEGAL_ARGUMENT)
        }
    }
}