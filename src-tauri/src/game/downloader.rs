use chksum::prelude::*;
use futures_util::StreamExt;
use reqwest::{Client, Url};
use serde::Serialize;
use std::{cmp::min, fs::File, io::Write, path::PathBuf};

pub const URL: &str = "";
pub const CHECKSUM: (&str, HashAlgorithm) = ("", HashAlgorithm::SHA2_256);

// TODO Нужно сделать checksum для АРХИВА и РАСПАКОВАННОЙ ПАПКИ ОТДЕЛЬНО!

const PATH: &str = ".mine-schizophrenia";

#[derive(Clone, Serialize, Debug)]
pub enum Progress {
    Downloading(u64),
    Decompressing,
}

pub fn get_path() -> PathBuf {
    dirs::config_local_dir().unwrap().join(PATH)
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
            dest: get_path(),
            client: Client::new(),
            callback: Box::new(|_| {}),
        }
    }
}

impl<'a> Downloader<'a> {
    pub async fn download(&mut self) -> anyhow::Result<()> {
        let mut archive = self.download_archive().await?;

        if format!("{:x}", archive.chksum(CHECKSUM.1)?) != CHECKSUM.0 {
            anyhow::bail!("invalid checksum: {}", CHECKSUM.0);
        }

        (self.callback)(Progress::Decompressing);
        sevenz_rust::decompress(archive, self.dest.clone())?;

        Ok(())
    }

    pub fn set_callback(&mut self, callback: impl FnMut(Progress) + Send + Sync + 'a) {
        self.callback = Box::new(callback);
    }

    async fn download_archive(&mut self) -> anyhow::Result<File> {
        let res = self.client.get(self.url.clone()).send().await?;
        let total_size = res.content_length().unwrap();
        let mut file = tempfile::tempfile()?;

        let mut downloaded: u64 = 0;
        let mut stream = res.bytes_stream();

        while let Some(item) = stream.next().await {
            let chunk = item?;
            file.write_all(&chunk)?;
            let new = min(downloaded + (chunk.len() as u64), total_size);
            downloaded = new;
            
            (self.callback)(Progress::Downloading(downloaded));
        }

        Ok(file)
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