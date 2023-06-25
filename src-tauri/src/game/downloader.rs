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
pub const URL: &str = "https://objects.githubusercontent.com/github-production-release-asset-2e65be/634833128/60223fb1-fc56-483d-8d80-1a52f7fa84c7?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIAIWNJYAX4CSVEH53A%2F20230625%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20230625T130930Z&X-Amz-Expires=300&X-Amz-Signature=74652522c7dd228fa5ae81d39498710827bb40c746f3074953c31a5fbed924a5&X-Amz-SignedHeaders=host&actor_id=116446344&key_id=0&repo_id=634833128&response-content-disposition=attachment%3B%20filename%3Dmine-linux.7z&response-content-type=application%2Foctet-stream";

#[cfg(target_os = "windows")]
pub const URL: &str = "https://objects.githubusercontent.com/github-production-release-asset-2e65be/634833128/cf7d99b9-79d5-47eb-b9dd-1c0f5427992b?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIAIWNJYAX4CSVEH53A%2F20230625%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20230625T130852Z&X-Amz-Expires=300&X-Amz-Signature=0dfe805308b1385a7fbc70ede3cfc840d9b03716196b9058d69e71729aea9a57&X-Amz-SignedHeaders=host&actor_id=116446344&key_id=0&repo_id=634833128&response-content-disposition=attachment%3B%20filename%3Dmine-windows.7z&response-content-type=application%2Foctet-stream";

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
