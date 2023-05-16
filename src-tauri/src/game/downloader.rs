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
use anyhow::Context;

pub struct Checksum<'a> {
    hash: &'a str,
    algorithm: HashAlgorithm,
}

impl<'a> Checksum<'a> {
    pub fn check(&self, file: &mut impl Chksum) -> anyhow::Result<bool> {
        Ok(format!("{:x}", file.chksum(self.algorithm)?) == self.hash)
    }
}

pub const URL: &str = "https://s80vlx.storage.yandex.net/rdisk/0d7c397d540254543b226d62a218ede08546a00d43648de9e2ac680d58714d62/6463f557/ebcgY3rvPKsNXoZfg1J4bWcoR8eCr1GwC2HiwemnhJnu936IH-HgYtxh8er7OhS0GAUGb3bTTLBtvGsf7Cgdwg==?uid=0&filename=minecra.7z&disposition=attachment&hash=ccmjnRHhAR8Dh18tCkeQX0GZNl0Xjin5yMnWf2A4UvIQ/AqL6mcvncq03KDH6RkUq/J6bpmRyOJonT3VoXnDag%3D%3D&limit=0&content_type=application%2Fx-7z-compressed&owner_uid=450618812&fsize=137049779&hid=b54a00c54b0ede2423cd28f37c630c71&media_type=compressed&tknv=v2&rtoken=3Emgxb4u1cPh&force_default=no&ycrid=na-cc5451d59196aea1b5e5c34bedced456-downloader20f&ts=5fbd63e56c3c0&s=3d21ffdc4937b2fe527ef67484f19e13f1b989868b9392a79988256295e2cebf&pb=U2FsdGVkX1_XBAEYSrw3vb6LEE1WoKG0OURNUdg5PDwm8eKDPJA-XJcVxTBkFd_eEjvJ2p7ua824QXu-3Iycrfngs9jt_aGFA3Wl_fRT1hA";

pub const CHECKSUM_FOR_ARCHIVE: Checksum = Checksum {
    hash: "1eb21cca15e2776a1d7c90bbe592ae840d4d91b0057788bce4e5b11723838dec",
    algorithm: HashAlgorithm::SHA2_256,
};

// TODO Нужно сделать checksum для АРХИВА и РАСПАКОВАННОЙ ПАПКИ ОТДЕЛЬНО!

const PATH: &str = "mine-schizophrenia";

#[derive(Clone, Debug, Serialize)]
pub struct DecompressStream<'a> {
    name: &'a str,
    size: u64,
    len_files: usize,
}

#[derive(Clone, Debug, Serialize)]
pub enum Progress<'a> {
    Downloading(u64),
    Decompressing(DecompressStream<'a>),
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
        let (mut archive, archive_path) = self.download_archive().await.context("downloading archive")?;

        if CHECKSUM_FOR_ARCHIVE.check(&mut archive)? {
            anyhow::bail!("invalid checksum archive");
        }

        if self.dest.is_dir() {
            fs::remove_dir_all(&self.dest)?;
        }

        fs::create_dir(&self.dest)?;

        let len_files = sevenz_rust::Archive::read(&mut File::open(&archive_path)?, 1024, b"").context("reading archive")?
            .files
            .len();

        sevenz_rust::decompress_file_with_extract_fn(
            archive_path,
            self.dest.clone(),
            |entry, reader, dest| {
                (self.callback)(Progress::Decompressing( DecompressStream { name: entry.name(), size: entry.size(), len_files }));
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

            (self.callback)(Progress::Downloading(downloaded));
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
