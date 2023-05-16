use chksum::prelude::*;
use futures_util::StreamExt;
use reqwest::{Client, Url};
use serde::Serialize;
use std::{cmp::min, fs::{File, self}, io::Write, path::PathBuf};

pub struct Checksum<'a> {
    hash: &'a str,
    algorithm: HashAlgorithm
}

impl<'a> Checksum<'a> {
    pub fn check(&self, file: &mut impl Chksum) -> anyhow::Result<bool> {
        Ok(format!("{:x}", file.chksum(self.algorithm)?) == self.hash)
    }
}

pub const URL: &str = "https://s596sas.storage.yandex.net/rdisk/34a416385b088576bd0ec33eb41d555f4f0b546be21d6c2a4acf74298ad2f2d6/6463a6e3/ebcgY3rvPKsNXoZfg1J4bWcoR8eCr1GwC2HiwemnhJnu936IH-HgYtxh8er7OhS0GAUGb3bTTLBtvGsf7Cgdwg==?uid=0&filename=minecra.7z&disposition=attachment&hash=ccmjnRHhAR8Dh18tCkeQX0GZNl0Xjin5yMnWf2A4UvIQ/AqL6mcvncq03KDH6RkUq/J6bpmRyOJonT3VoXnDag%3D%3D&limit=0&content_type=application%2Fx-7z-compressed&owner_uid=450618812&fsize=137049779&hid=b54a00c54b0ede2423cd28f37c630c71&media_type=compressed&tknv=v2&rtoken=J6EJ4o7Msocw&force_default=no&ycrid=na-c3f0533855ae0bc877ac25a6d0c29e38-downloader15e&ts=5fbd1913d3ec0&s=67ca0b6ab7d7e3b82fb0aa904d7d2571e97a1b173cc8a0f5872519714bdf3fc2&pb=U2FsdGVkX1_vdPw_wWSdrAR53XRkbyzlMZE2SzgQgux0XsL9j9Pvpf7N0ZjaxIMNGTdvB42clas83rzLRU790QdoRSOqNS0HGqTvyVgGHYk";

pub const CHECKSUM_FOR_ARCHIVE: Checksum = Checksum {
    hash: "",
    algorithm: HashAlgorithm::SHA2_256
};

pub const CHECKSUM_FOR_UNPACKED_ARCHIVE: Checksum = Checksum {
    hash: "",
    algorithm: HashAlgorithm::SHA2_256
};

// TODO Нужно сделать checksum для АРХИВА и РАСПАКОВАННОЙ ПАПКИ ОТДЕЛЬНО!

const PATH: &str = "mine-schizophrenia";

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
        let mut archive = tempfile::tempfile()?; // BUG /tmp/#165 (deleted)
        writeln!(archive, "Brian was here. Briefly.")?;
        log::error!("arvv: {:?}", archive);
        self.download_archive(&mut archive).await?;

        if CHECKSUM_FOR_ARCHIVE.check(&mut archive)? {
            anyhow::bail!("invalid checksum archive");
        }

        (self.callback)(Progress::Decompressing);

        if self.dest.is_dir() {
            fs::remove_dir_all(&self.dest)?;
        }

        fs::create_dir(&self.dest)?;
        log::error!("archive: {:?}", archive);
        sevenz_rust::decompress_with_extract_fn(archive, self.dest.clone(), |i, a, p| {
            println!("{:?}, {:?}", i, p);
            sevenz_rust::default_entry_extract_fn(i, a, p)
        })?;

        Ok(())
    }

    pub fn set_callback(&mut self, callback: impl FnMut(Progress) + Send + Sync + 'a) {
        self.callback = Box::new(callback);
    }

    async fn download_archive(&mut self, file: &mut File) -> anyhow::Result<()> {
        let res = self.client.get(self.url.clone()).send().await?;
        let total_size = res.content_length().unwrap();

        let mut downloaded: u64 = 0;
        let mut stream = res.bytes_stream();

        while let Some(item) = stream.next().await {
            let chunk = item?;
            file.write_all(&chunk)?;
            let new = min(downloaded + (chunk.len() as u64), total_size);
            downloaded = new;
            
            (self.callback)(Progress::Downloading(downloaded));
        }

        Ok(())
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