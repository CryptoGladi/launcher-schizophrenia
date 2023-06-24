use anyhow::Context;
use futures_util::StreamExt;
use reqwest::{Client, Url};
use serde::Serialize;
use std::env::temp_dir;
use std::{
    cmp::min,
    fs::{self, File, OpenOptions},
    io::Write,
    path::PathBuf,
};

#[cfg(target_os = "linux")]
pub const URL: &str = "https://s124vlx.storage.yandex.net/rdisk/c97f325f59e4ddac92dd6f389df08c2946c7d6b83b927e5151f99d6999468b1c/64931305/ebcgY3rvPKsNXoZfg1J4bdnwl8Bp2Gr_FZnPIWNdE44oF6trNxc_gPKvDolHBUO_ok6pHJ48rvGoc24f_nLNyg==?uid=0&filename=gtrhgsdvsd.7z&disposition=attachment&hash=GmNiElPuRqCvk6%2B9E2vLzUMdNRgzAI41U3MlJiNiPUtsO5G6Na1zwcdioWxCzALhq/J6bpmRyOJonT3VoXnDag%3D%3D&limit=0&content_type=application%2Fx-7z-compressed&owner_uid=450618812&fsize=933524746&hid=78abe62232271b50d5166bd8d8e07a72&media_type=compressed&tknv=v2&rtoken=ebCWop2dN11H&force_default=no&ycrid=na-e7a7f4d350306e71426ef601fc49d024-downloader21f&ts=5fea52cf70b40&s=f7d8503229cbf03596edce9162f9c3729cbc037334807ab444f041067f87de21&pb=U2FsdGVkX18dUKutBkIyXlak9lUzQeAuG0y9Unfzo26cSNqzK26opjDyVN-o51Sh9NJYIaf2f6jIAv4AW-N0oaLEDLNFFBenXiB060OfkjM";

#[cfg(target_os = "windows")]
pub const URL: &str = "https://s118vlx.storage.yandex.net/rdisk/3c0c9eb53a96b7317a23dc85fee64f0954c59557abe37d04ffe10e210e622f3d/6495c9f6/ebcgY3rvPKsNXoZfg1J4baSu7RZ33l-N7worf_Zn6kXcHbPhJYg0wKuyx8lkxF7AJRD0r47jAKJFPMGzWTJjFQ==?uid=0&filename=windows.7z&disposition=attachment&hash=rvSOufaXK94E3ZY33UTM%2BIFJgkCWeEmPX50esHO%2BwEn4jzqTC4eBo%2BYrZpyZBTHCq/J6bpmRyOJonT3VoXnDag%3D%3D&limit=0&content_type=application%2Fx-7z-compressed&owner_uid=450618812&fsize=1007990517&hid=19d56e31331bcbc5de7efa6a2ce504fe&media_type=compressed&tknv=v2&rtoken=E2q1QoJZ4kw6&force_default=no&ycrid=na-bd2a14db706a321e6c4bd51b7edd57d4-downloader20f&ts=5fece98ede980&s=a19950780f8dd549c03de7c5d76f47a9cf57be168d43d3c955749f1e6a7187d6&pb=U2FsdGVkX1-P4-_FjQD0qplcUq3nvDP8DHpzTJjv2IPvnUTL9b8uUPNZH3IoGoU0ugQaJ8H84Zt8Ur5ZsGq4jHRjmmueWIUpeZtx9X_MS3Y";

#[derive(Clone, Debug, Serialize)]
pub struct DecompressStream<'a> {
    name: &'a str,
    size: u64,
    len_done_files: usize,
    total_files: usize,
}

#[derive(Clone, Debug, Serialize)]
pub struct DownloadStream {
    percent_done: u64,
}

#[derive(Clone, Debug, Serialize)]
pub enum Progress<'a> {
    Downloading(DownloadStream),
    Decompressing(DecompressStream<'a>),
}

pub struct Downloader<'a> {
    pub url: Url,
    pub dest: PathBuf,
    client: Client,
    callback: Box<dyn FnMut(Progress) + Send + Sync + 'a>,
}

impl<'a> Default for Downloader<'a> {
    fn default() -> Self {
        Self {
            url: URL.parse().unwrap(),
            dest: crate::path::get_app_folder(),
            client: Client::new(),
            callback: Box::new(|_| {}),
        }
    }
}

impl<'a> Downloader<'a> {
    pub async fn download(&mut self) -> anyhow::Result<()> {
        let (_, archive_path) = self
            .download_archive()
            .await
            .context("downloading archive")?;

        if self.dest.is_dir() {
            remove_dir_all::remove_dir_contents(&self.dest)?;
        }

        fs::create_dir(&self.dest)?;

        let len_files = sevenz_rust::Archive::read(&mut File::open(&archive_path)?, 1024, b"")
            .context("reading archive")?
            .files
            .len();

        let mut len_done_files = 0;
        sevenz_rust::decompress_file_with_extract_fn(
            archive_path,
            self.dest.clone(),
            |entry, reader, dest| {
                (self.callback)(Progress::Decompressing(DecompressStream {
                    name: entry.name(),
                    size: entry.size(),
                    total_files: len_files,
                    len_done_files,
                }));
                len_done_files += 1;
                sevenz_rust::default_entry_extract_fn(entry, reader, dest)
            },
        )?;

        Ok(())
    }

    pub fn set_callback(&mut self, callback: impl FnMut(Progress) + Send + Sync + 'a) {
        self.callback = Box::new(callback);
    }

    async fn download_archive(&mut self) -> anyhow::Result<(File, PathBuf)> {
        let path = temp_dir().join("downloaded-mine-schizophrenia.7z");

        if path.is_file() {
            fs::remove_file(&path)?;
        }

        let res = self.client.get(self.url.clone()).send().await?;

        if !res.status().is_success() {
            anyhow::bail!("response to download_archive. status: {}", res.status())
        }

        let total_size = res.content_length().unwrap();
        let mut file = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(&path)?;

        let mut downloaded: u64 = 0;
        let mut stream = res.bytes_stream();

        while let Some(item) = stream.next().await {
            let chunk = item?;
            file.write_all(&chunk)?;
            let new = min(downloaded + (chunk.len() as u64), total_size);
            downloaded = new;

            (self.callback)(Progress::Downloading(DownloadStream {
                percent_done: (downloaded * 100) / total_size,
            }));
        }

        Ok((file, path))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn check_url() {
        let client = reqwest::Client::new();
        let res = client.get(URL).send().await.unwrap();

        assert_eq!(res.status().is_success(), true);
    }
}
