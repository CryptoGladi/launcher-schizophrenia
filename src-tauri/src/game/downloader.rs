use anyhow::Context;
use futures_util::StreamExt;
use reqwest::cookie::Jar;
use reqwest::{Client, ClientBuilder, Url};
use serde::Serialize;
use std::env::temp_dir;
use std::{
    cmp::min,
    fs::{self, File, OpenOptions},
    io::Write,
    path::PathBuf,
};

// https://stackoverflow.com/questions/25010369/wget-curl-large-file-from-google-drive

#[cfg(target_os = "linux")]
pub const URL: &str = r"https://drive.google.com/u/0/uc?id=16RqO23hPdP9vh0-jilKJdAYfvg-FaHMp&export=download&confirm=t&uuid=d62d3c0e-8ec5-4017-97d5-ef49bfe8806e&at=AKKF8vyEgM2Cs5XmGvC0GAhorm1o:1687948797802";

#[cfg(target_os = "windows")]
pub const URL: &str = r"https://drive.google.com/u/0/uc?id=1AiEruVf_v1LnGZLMdFP0M3Lcy89N4kPr&export=download&confirm=t&uuid=f92a16c0-395f-4aae-a3eb-91a44cc834ec&at=AKKF8vz4XTZmu8ArZu5j1OMrjTx8:1687949189969";

fn get_url() -> Result<String> {
    #[cfg(target_os = "linux")]
    {

    }
    todo!()
}

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
            client: {
                let cookie = "foo=bar; Domain=yolo.local";
                let jar = Jar::default();

                jar.add_cookie_str(cookie, &URL.parse().unwrap());

                let client = ClientBuilder::new()
                    .cookie_store(true)
                    .cookie_provider(jar.into())
                    .build()
                    .unwrap();
                client
            },
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

        //fs::create_dir(&self.dest)?;

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
