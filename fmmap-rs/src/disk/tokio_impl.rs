use std::path::{Path, PathBuf};
#[cfg(not(target_os = "linux"))]
use std::ptr::{drop_in_place, write};
use async_trait::async_trait;
use crate::MetaData;
use crate::tokio::{AsyncMmapFileExt, AsyncMmapFileMutExt, AsyncOptions};
use crate::disk::{MmapFileMutType, remmap};
use crate::error::Error;
use crate::utils::tokio::{create_file_async, open_exist_file_with_append_async, open_or_create_file_async, open_read_only_file_async, sync_dir_async};
use fs4::tokio::AsyncFileExt;
use memmap2::{Mmap, MmapMut, MmapOptions};
use tokio::fs::{File, remove_file};

declare_and_impl_async_fmmap_file!("tokio_async", "tokio_test", "tokio", File);

declare_and_impl_async_fmmap_file_mut!("tokio_async", "tokio_test", "tokio", File, AsyncDiskMmapFile);

#[cfg(test)]
mod test {
    use super::*;
    use scopeguard::defer;

    #[tokio::test]
    async fn test_close_with_truncate_on_empty_file() {
        let file = AsyncDiskMmapFileMut::create("tokio_async_disk_close_with_truncate_test.txt").await.unwrap();
        defer!(std::fs::remove_file("tokio_async_disk_close_with_truncate_test.txt").unwrap());
        file.close_with_truncate(10).await.unwrap();

        assert_eq!(10, File::open("tokio_async_disk_close_with_truncate_test.txt").await.unwrap().metadata().await.unwrap().len());
    }
}