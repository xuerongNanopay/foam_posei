#![allow(unused)]

use crate::internal::FPErr;

// Common
pub const FP_NO_IMPL:          FPErr  = -1;
pub const FP_NO_SUPPORT:       FPErr  = -2;
pub const FP_NO_FOUND:         FPErr  = -3;
pub const FP_ILLEGAL_ARGUMENT: FPErr  = -4;
pub const FP_BUSY:             FPErr  = -5;
pub const FP_RETRY:            FPErr  = -6;

pub const FP_NO_ERR:           FPErr  = 0;

// Mem.
pub const FP_ALLOC_FAIL: FPErr           = 1;


#[macro_export]
macro_rules! FP_IO_ERR_RET {
    ($func:expr) => {
        match $func {
            Ok(e) => e,
            Err(e) => return Err(crate::error::convert_std_io_err_to_fp_err(e)),
        }
    };
}

#[macro_export]
macro_rules! FP_ASSERT {
    ($option:expr, $err:expr) => {
        if $option {
            return Err($err);
        }
    };
}

#[macro_export]
macro_rules! FP_ERR_RET {
    ($func:expr, $e:expr) => {
        match $func {
            Ok(o) => o,
            Err(e) => return Err($e),
        }
    };
}

#[macro_export]
macro_rules! FP_ASSERT_NOT_NONE {
    ($option:expr, $err:expr) => {
        match $option {
            Some(o) => o,
            None =>return Err($err),
        }
    };
    ($option:expr) => {
        FP_ASSERT_NOT_NONE!($option, crate::error::FP_ILLEGAL_ARGUMENT)
    };
}

#[macro_export]
macro_rules! FP_ASSERT_NONE {
    ($option:expr, $err:expr) => {
        match $option {
            Some(o) => eturn Err($err),
            None =>,
        }
    };
    ($option:expr) => {
        FP_ASSERT_NOT_NONE!($option, crate::error::FP_ILLEGAL_ARGUMENT)
    };
}

// IO errors.
pub const FP_IO_NOT_FOUND:                 FPErr = 101;
pub const FP_IO_PERMISSION_DENIED:         FPErr = 102;
pub const FP_IO_CONNECTION_REFUSED:        FPErr = 103;
pub const FP_IO_CONNECTION_RESET:          FPErr = 104;
pub const FP_IO_HOST_UNREACHABLE:          FPErr = 105;
pub const FP_IO_NETWORK_UNREACHABLE:       FPErr = 106;
pub const FP_IO_CONNECTION_ABORTED:        FPErr = 107;
pub const FP_IO_NOT_CONNECTED:             FPErr = 108;
pub const FP_IO_ADDR_IN_USE:               FPErr = 109;
pub const FP_IO_ADDR_NOT_AVAILABLE:        FPErr = 110;
pub const FP_IO_NETWORK_DOWN:              FPErr = 111;
pub const FP_IO_BROKEN_PIPE:               FPErr = 112;
pub const FP_IO_ALREADY_EXISTS:            FPErr = 113;
pub const FP_IO_WOULD_BLOCK:               FPErr = 114;
pub const FP_IO_NOT_A_DIRECTORY:           FPErr = 115;
pub const FP_IO_IS_A_DIRECTORY:            FPErr = 116;
pub const FP_IO_DIRECTORY_NOT_EMPTY:       FPErr = 117;
pub const FP_IO_READ_ONLY_FILESYSTEM:      FPErr = 118;
pub const FP_IO_FILESYSTEM_LOOP:           FPErr = 119;
pub const FP_IO_STALE_NETWORK_FILE_HANDLE: FPErr = 120;
pub const FP_IO_INVALID_INPUT:             FPErr = 121;
pub const FP_IO_INVALID_DATA:              FPErr = 122;
pub const FP_IO_TIMED_OUT:                 FPErr = 123;
pub const FP_IO_WRITE_ZERO:                FPErr = 124;
pub const FP_IO_STORAGE_FULL:              FPErr = 125;
pub const FP_IO_NOT_SEEKABLE:              FPErr = 126;
pub const FP_IO_FILESYSTEM_QUOTA_EXCEEDED: FPErr = 127;
pub const FP_IO_FILE_TOO_LARGE:            FPErr = 128;
pub const FP_IO_RESOURCE_BUSY:             FPErr = 129;
pub const FP_IO_EXECUTABLE_FILE_BUSY:      FPErr = 130;
pub const FP_IO_DEAD_LOCK:                 FPErr = 131;
pub const FP_IO_CROSSES_DEVICES:           FPErr = 132;
pub const FP_IO_TOO_MANY_LINKS:            FPErr = 133;
pub const FP_IO_INVALID_FILENAME:          FPErr = 134;
pub const FP_IO_ARGUMENT_LIST_TOO_LONG:    FPErr = 135;
pub const FP_IO_INTERRUPTED:               FPErr = 136;
pub const FP_IO_UNSUPPORTED:               FPErr = 137;
pub const FP_IO_UNEXPECTED_EOF:            FPErr = 138;
pub const FP_IO_OUT_OF_MEMORY:             FPErr = 139;
pub const FP_IO_IN_PROGRESS:               FPErr = 140;
pub const FP_IO_OTHER:                     FPErr = 141;
pub const FP_IO_UNKNOWN_ERR:               FPErr = 142;

// block errors.
pub const FP_BK_DATA_CORRUPTION:    FPErr = 201;
pub const FP_BK_INVALID_MAGIC:      FPErr = 202;
pub const FP_BK_INVALID_MAJOR:      FPErr = 203;
pub const FP_BK_INVALID_MINOR:      FPErr = 204;
pub const FP_BK_ILLEGAL_ARGUMENT:   FPErr = 205;
pub const FP_BK_ILLEGAL_BLOCK_SIZE: FPErr = 206;
pub const FP_BK_ILLEGAL_CHECKSUM:   FPErr = 207;

pub fn convert_std_io_err_to_fp_err(ioe: std::io::Error) -> FPErr {
    match ioe.kind() {
        std::io::ErrorKind::NotFound               => FP_IO_NOT_FOUND,
        std::io::ErrorKind::PermissionDenied       => FP_IO_PERMISSION_DENIED,
        std::io::ErrorKind::ConnectionRefused      => FP_IO_CONNECTION_REFUSED,
        std::io::ErrorKind::ConnectionReset        => FP_IO_CONNECTION_RESET,
        std::io::ErrorKind::HostUnreachable        => FP_IO_HOST_UNREACHABLE,
        std::io::ErrorKind::NetworkUnreachable     => FP_IO_NETWORK_UNREACHABLE,
        std::io::ErrorKind::ConnectionAborted      => FP_IO_CONNECTION_ABORTED,
        std::io::ErrorKind::NotConnected           => FP_IO_NOT_CONNECTED,
        std::io::ErrorKind::AddrInUse              => FP_IO_ADDR_IN_USE,
        std::io::ErrorKind::AddrNotAvailable       => FP_IO_ADDR_NOT_AVAILABLE,
        std::io::ErrorKind::NetworkDown            => FP_IO_NETWORK_DOWN,
        std::io::ErrorKind::BrokenPipe             => FP_IO_BROKEN_PIPE,
        std::io::ErrorKind::AlreadyExists          => FP_IO_ALREADY_EXISTS,
        std::io::ErrorKind::WouldBlock             => FP_IO_WOULD_BLOCK,
        std::io::ErrorKind::NotADirectory          => FP_IO_NOT_A_DIRECTORY,
        std::io::ErrorKind::IsADirectory           => FP_IO_IS_A_DIRECTORY,
        std::io::ErrorKind::DirectoryNotEmpty      => FP_IO_DIRECTORY_NOT_EMPTY,
        std::io::ErrorKind::ReadOnlyFilesystem     => FP_IO_READ_ONLY_FILESYSTEM,
        // std::io::ErrorKind::FilesystemLoop      => FP_IO_FILESYSTEM_LOOP,
        std::io::ErrorKind::StaleNetworkFileHandle => FP_IO_STALE_NETWORK_FILE_HANDLE,
        std::io::ErrorKind::InvalidInput           => FP_IO_INVALID_INPUT,
        std::io::ErrorKind::InvalidData            => FP_IO_INVALID_DATA,
        std::io::ErrorKind::TimedOut               => FP_IO_TIMED_OUT,
        std::io::ErrorKind::WriteZero              => FP_IO_WRITE_ZERO,
        std::io::ErrorKind::StorageFull            => FP_IO_STORAGE_FULL,
        std::io::ErrorKind::NotSeekable            => FP_IO_NOT_SEEKABLE,
        // std::io::ErrorKind::FilesystemQuotaExceeded => FP_IO_FILESYSTEM_QUOTA_EXCEEDED,
        std::io::ErrorKind::FileTooLarge           => FP_IO_FILE_TOO_LARGE,
        std::io::ErrorKind::ResourceBusy           => FP_IO_RESOURCE_BUSY,
        std::io::ErrorKind::ExecutableFileBusy     => FP_IO_EXECUTABLE_FILE_BUSY,
        std::io::ErrorKind::Deadlock               => FP_IO_DEAD_LOCK,
        // std::io::ErrorKind::CrossesDevices => FP_IO_CROSSES_DEVICES,
        std::io::ErrorKind::TooManyLinks           => FP_IO_TOO_MANY_LINKS,
        // std::io::ErrorKind::InvalidFilename => FP_IO_INVALID_FILENAME,
        std::io::ErrorKind::ArgumentListTooLong    => FP_IO_ARGUMENT_LIST_TOO_LONG,
        std::io::ErrorKind::Interrupted            => FP_IO_INTERRUPTED,
        std::io::ErrorKind::Unsupported            => FP_IO_UNSUPPORTED,
        std::io::ErrorKind::UnexpectedEof          => FP_IO_UNEXPECTED_EOF,
        std::io::ErrorKind::OutOfMemory            => FP_IO_OUT_OF_MEMORY,
        // ErrorKind::InProgress => FP_IO_IN_PROGRESS,
        std::io::ErrorKind::Other                  => FP_IO_OTHER,
        //TODO: log
        _ => FP_IO_UNKNOWN_ERR,
    }
}

pub const FP_BT_ROW_KEY_MEM_INIT:        FPErr = 301;
pub const FP_BT_PAGE_READ_NOT_FOUND:     FPErr = 302;
pub const FP_BT_PAGE_READ_RETRY:         FPErr = 302;

pub const FP_BLK_HDL_READ_ILL_BLK_SIZE: FPErr = 501;
pub const FP_BLK_HDL_READ_ILL_CHECKSUM: FPErr = 502;

pub const FP_BTREE_PAGE_HEADER_LEN_ILL: FPErr = 11001;
pub const FP_BTREE_PAGE_TYPE_ILL:       FPErr = 11002;