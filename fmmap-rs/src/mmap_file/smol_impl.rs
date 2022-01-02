use std::borrow::Cow;
use std::mem;
use std::path::{Path, PathBuf};
use async_trait::async_trait;
use smol::io::{Cursor, AsyncWriteExt};
use crate::smol::{AsyncMmapFileReader, AsyncMmapFileWriter, AsyncOptions};
use crate::disk::smol_impl::{AsyncDiskMmapFile, AsyncDiskMmapFileMut};
use crate::empty::smol_impl::AsyncEmptyMmapFile;
use crate::error::{Error, Result};
use crate::memory::smol_impl::{AsyncMemoryMmapFile, AsyncMemoryMmapFileMut};
use crate::metadata::MetaData;

declare_async_mmap_file_ext!(AsyncDiskMmapFileMut, AsyncOptions, AsyncMmapFileReader);

declare_async_mmap_file_mut_ext!(AsyncMmapFileWriter);

#[enum_dispatch(AsyncMmapFileExt)]
enum AsyncMmapFileInner {
    Empty(AsyncEmptyMmapFile),
    Memory(AsyncMemoryMmapFile),
    Disk(AsyncDiskMmapFile)
}

declare_and_impl_async_mmap_file!("smol_async", "smol", "smol");

#[enum_dispatch(AsyncMmapFileExt, AsyncMmapFileMutExt)]
enum AsyncMmapFileMutInner {
    Empty(AsyncEmptyMmapFile),
    Memory(AsyncMemoryMmapFileMut),
    Disk(AsyncDiskMmapFileMut)
}

delcare_and_impl_async_mmap_file_mut!("smol_async", "smol", "smol");