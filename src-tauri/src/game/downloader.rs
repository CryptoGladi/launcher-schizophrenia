use std::path::PathBuf;

use chksum::prelude::HashAlgorithm;
use reqwest::{Client, Url};
use anyhow::Result;

pub const URL: &str = "";
pub const CHECKSUM: (&str, HashAlgorithm) = ("", HashAlgorithm::SHA2_256);
const PATH: &str = ".mine-schizophrenia";

pub fn get_path() -> PathBuf {
    dirs::config_local_dir().unwrap().join(PATH)
}

pub struct Downloader {
    pub url: Url,
    client: Client,
}

impl Default for Downloader {
    fn default() -> Self {
        Self {
            url: URL.parse().unwrap(),
            client: Client::new(),
        }
    }
}

impl Downloader {
    async fn download(&self) -> anyhow::Result<()> {
        let res = self.client.get(self.url.clone()).send().await?;

        let total_size = res.content_length().unwrap();

        // Indicatif setup
        //let pb = ProgressBar::new(total_size);
        //pb.set_style(ProgressStyle::default_bar()
        //    .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        //    .progress_chars("#>-"));
        //spb.set_message(&format!("Downloading {}", url));

        // download chunks
        /* 
        let mut file = File::create(path).or(Err(format!("Failed to create file '{}'", path)))?;
        let mut downloaded: u64 = 0;
        let mut stream = res.bytes_stream();

        while let Some(item) = stream.next().await {
            let chunk = item.or(Err(format!("Error while downloading file")))?;
            file.write_all(&chunk)
                .or(Err(format!("Error while writing to file")))?;
            let new = min(downloaded + (chunk.len() as u64), total_size);
            downloaded = new;
            pb.set_position(new);
        }
        */

        Ok(())
    }
}
