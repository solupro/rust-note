use anyhow::{anyhow, Result};
use async_trait::async_trait;
use tokio::fs;

#[async_trait]
pub trait Fetch {
  type Error;
  async fn fetch(&self) -> Result<String, Self::Error>;
}

struct UrlFetch<'a> (pub(crate) &'a str);
struct FileFetch<'a> (pub(crate) &'a str);

#[async_trait]
impl<'a> Fetch for UrlFetch<'a> {
  type Error = anyhow::Error;

  async fn fetch(&self) -> Result<String, Self::Error> {
    Ok(reqwest::get(self.0).await?.text().await?)
  }
}

#[async_trait]
impl<'a> Fetch for FileFetch<'a> {
  type Error = anyhow::Error;

  async fn fetch(&self) -> Result<String, Self::Error> {
    Ok(fs::read_to_string(&self.0[7..]).await?)
  }
}

pub async fn retrieve_data(source: impl AsRef<str>) -> Result<String> {
  let name = source.as_ref();
  match &name[..4] {
    "http" => UrlFetch(name).fetch().await,
    "file" => FileFetch(name).fetch().await,
    _ => return Err(anyhow!("We only support http[s]/file at the moment")),
  }
}