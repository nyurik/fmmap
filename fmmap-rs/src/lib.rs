//! A high level wrapper for memory map with flexible and convenient methods.
#![cfg_attr(feature = "nightly", feature(is_symlink), feature(io_error_more))]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![allow(
    rustdoc::broken_intra_doc_links,
    unused_macros,
    clippy::len_without_is_empty,
    clippy::upper_case_acronyms
)]

#[macro_use]
extern crate enum_dispatch;

macro_rules! cfg_smol {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "smol-async")]
            #[cfg_attr(docsrs, doc(cfg(feature = "smol-async")))]
            $item
        )*
    }
}

macro_rules! cfg_tokio {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "tokio-async")]
            #[cfg_attr(docsrs, doc(cfg(feature = "tokio-async")))]
            $item
        )*
    }
}

macro_rules! cfg_sync {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "sync")]
            #[cfg_attr(docsrs, doc(cfg(feature = "sync")))]
            $item
        )*
    }
}

macro_rules! cfg_windows {
    ($($item:item)*) => {
        $(
            #[cfg(windows)]
            #[cfg_attr(docsrs, doc(cfg(windows)))]
            $item
        )*
    }
}

macro_rules! cfg_unix {
    ($($item:item)*) => {
        $(
            #[cfg(unix)]
            #[cfg_attr(docsrs, doc(cfg(unix)))]
            $item
        )*
    }
}

macro_rules! noop_flush {
    () => {
        #[inline(always)]
        fn flush(&self) -> crate::error::Result<()> {
            Ok(())
        }

        #[inline(always)]
        fn flush_async(&self) -> crate::error::Result<()> {
            Ok(())
        }

        #[inline(always)]
        fn flush_range(&self, _offset: usize, _len: usize) -> crate::error::Result<()> {
            Ok(())
        }

        #[inline(always)]
        fn flush_async_range(&self, _offset: usize, _len: usize) -> crate::error::Result<()> {
            Ok(())
        }
    };
}

mod disk;
mod empty;
/// Errors in this crate
pub mod error;
mod memory;
mod metadata;
mod mmap_file;
#[allow(dead_code)]
mod options;
mod reader;
/// File I/O utils function
pub mod utils;
mod writer;

cfg_sync!(
    pub use reader::MmapFileReader;
    pub use writer::MmapFileWriter;
    pub use mmap_file::{MmapFileExt, MmapFileMutExt, MmapFile, MmapFileMut};
    pub use options::Options;
);

cfg_tokio!(
    #[macro_use]
    extern crate async_trait;
    pub use reader::tokio_impl::AsyncMmapFileReader;
    pub use writer::tokio_impl::AsyncMmapFileWriter;
    pub use mmap_file::{AsyncMmapFileExt, AsyncMmapFileMutExt, AsyncMmapFile, AsyncMmapFileMut};
    pub use options::AsyncOptions;
);

pub use metadata::{MetaData, MetaDataExt};

/// Inner structs of [`MmapFile`], [`MmapFileMut`], [`AsyncMmapFile`], [`AsyncMmapFileMut`]
///
/// [`MmapFile`]: structs.MmapFile.html
/// [`MmapFileMut`]: structs.MmapFileMut.html
/// [`AsyncMmapFile`]: structs.AsyncMmapFile.html
/// [`AsyncMmapFileMut`]: structs.AsyncMmapFileMut.html
pub mod raw {
    cfg_sync!(
        pub use crate::disk::{DiskMmapFile, DiskMmapFileMut};
        pub use crate::memory::{MemoryMmapFile, MemoryMmapFileMut};
    );

    cfg_tokio!(
        pub use crate::disk::{AsyncDiskMmapFile, AsyncDiskMmapFileMut};
        pub use crate::memory::{AsyncMemoryMmapFile, AsyncMemoryMmapFileMut};
    );
}
