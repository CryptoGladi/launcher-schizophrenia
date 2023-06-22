use anyhow::Context;
use chksum::prelude::*;
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

pub struct Checksum<'a> {
    hash: &'a str,
    algorithm: HashAlgorithm,
}

impl<'a> Checksum<'a> {
    pub fn check(&self, file: &mut impl Chksum) -> anyhow::Result<bool> {
        Ok(format!("{:x}", file.chksum(self.algorithm)?) == self.hash)
    }
}

#[cfg(target_os = "linux")]
pub const URL: &str = "https://s124vlx.storage.yandex.net/rdisk/c97f325f59e4ddac92dd6f389df08c2946c7d6b83b927e5151f99d6999468b1c/64931305/ebcgY3rvPKsNXoZfg1J4bdnwl8Bp2Gr_FZnPIWNdE44oF6trNxc_gPKvDolHBUO_ok6pHJ48rvGoc24f_nLNyg==?uid=0&filename=gtrhgsdvsd.7z&disposition=attachment&hash=GmNiElPuRqCvk6%2B9E2vLzUMdNRgzAI41U3MlJiNiPUtsO5G6Na1zwcdioWxCzALhq/J6bpmRyOJonT3VoXnDag%3D%3D&limit=0&content_type=application%2Fx-7z-compressed&owner_uid=450618812&fsize=933524746&hid=78abe62232271b50d5166bd8d8e07a72&media_type=compressed&tknv=v2&rtoken=ebCWop2dN11H&force_default=no&ycrid=na-e7a7f4d350306e71426ef601fc49d024-downloader21f&ts=5fea52cf70b40&s=f7d8503229cbf03596edce9162f9c3729cbc037334807ab444f041067f87de21&pb=U2FsdGVkX18dUKutBkIyXlak9lUzQeAuG0y9Unfzo26cSNqzK26opjDyVN-o51Sh9NJYIaf2f6jIAv4AW-N0oaLEDLNFFBenXiB060OfkjM";

#[cfg(target_os = "windows")]
pub const URL: &str = "https://s118vlx.storage.yandex.net/rdisk/3622a89f82f4d7885e747961b003af542627ca93adb2033835dfa5d89829a883/6494ca5b/ebcgY3rvPKsNXoZfg1J4baSu7RZ33l-N7worf_Zn6kXcHbPhJYg0wKuyx8lkxF7AJRD0r47jAKJFPMGzWTJjFQ==?uid=0&filename=windows.7z&disposition=attachment&hash=rvSOufaXK94E3ZY33UTM%2BIFJgkCWeEmPX50esHO%2BwEn4jzqTC4eBo%2BYrZpyZBTHCq/J6bpmRyOJonT3VoXnDag%3D%3D&limit=0&content_type=application%2Fx-7z-compressed&owner_uid=450618812&fsize=1007990517&hid=19d56e31331bcbc5de7efa6a2ce504fe&media_type=compressed&tknv=v2&rtoken=krR0FlnUpiHS&force_default=no&ycrid=na-198f5b8d96dc0cd6fef949fdf85f8d9d-downloader3h&ts=5febf5cb30cc0&s=84056f243864c1185a7276813ffc9afe424ba694c1c8f10f5a70e7fb16b87927&pb=U2FsdGVkX19cfC_rUyyxrbT6a7Bg13XRtW8GwT8U78f-O1Gj5uBGROwdjJEd_2COGukuiJ8nGb2piNkJkj8bd9wJ5f8i1TAsQcTTgN72xgM";

pub const CHECKSUM_FOR_ARCHIVE: Checksum = Checksum {
    hash: "1eb21cca15e2776a1d7c90bbe592ae840d4d91b0057788bce4e5b11723838dec",
    algorithm: HashAlgorithm::SHA2_256,
};

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
        let (mut archive, archive_path) = self
            .download_archive()
            .await
            .context("downloading archive")?;

        if CHECKSUM_FOR_ARCHIVE.check(&mut archive)? {
            anyhow::bail!("invalid checksum archive");
        }

        if self.dest.is_dir() {
            fs::remove_dir_all(&self.dest)?;
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
