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

pub const URL: &str = "https://s80vlx.storage.yandex.net/rdisk/97101d99791b78a5b7e4d5451d95f8d10ad42a6f2b9c32cb7a0ab5e42c5e1a45/648eead7/ebcgY3rvPKsNXoZfg1J4bWcoR8eCr1GwC2HiwemnhJnu936IH-HgYtxh8er7OhS0GAUGb3bTTLBtvGsf7Cgdwg==?uid=0&filename=minecra.7z&disposition=attachment&hash=ccmjnRHhAR8Dh18tCkeQX0GZNl0Xjin5yMnWf2A4UvIQ/AqL6mcvncq03KDH6RkUq/J6bpmRyOJonT3VoXnDag%3D%3D&limit=0&content_type=application%2Fx-7z-compressed&owner_uid=450618812&fsize=137049779&hid=b54a00c54b0ede2423cd28f37c630c71&media_type=compressed&tknv=v2&rtoken=7d676NJyAgJs&force_default=no&ycrid=na-0d51bd618f8edf0eb191ad5d13b6b2e2-downloader1f&ts=5fe65bedf23c0&s=f82a0a70638a12b2c34940441e31f37de944e8233c783754e98a873b01033580&pb=U2FsdGVkX19W_WPDAcTzvbCmv0RpiweJFfcWQjbbYJO8grI7tIqSIHYB4MZUYmQNei7GUq08Ld_6L_crWQHe3_QZjsNDZ8_9Hy_gctGC4mQ";

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
            dest: crate::path::get_config(),
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
