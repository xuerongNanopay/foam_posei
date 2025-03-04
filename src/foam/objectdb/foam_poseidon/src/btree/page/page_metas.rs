#![allow(unused)]

use crate::internal::{FPTimeStamp, FPTxnId};

/**
 * Page marker for delete.
 */
#[derive(Clone, Copy)]
pub(super) struct PageDeleted {
    txn_id:           u64,
    timestamp:        u64,
    commit_timestamp: u64,
    commited:         bool,
}

impl PageDeleted {
    pub(super) fn new (txn_id: u64, timestamp: u64, commit_timestamp: u64) -> Self {
        Self {
            txn_id,
            timestamp,
            commit_timestamp,
            commited: true,
        }
    }
}

/**
 * Work with MVCC
 */
pub(crate) struct PageAddrTS {
    initial_commit_ts: u64,
    end_commit_ts:     u64,
}

/**
 * Work with MVCC
 */
pub(crate) struct PageKVTS {

}