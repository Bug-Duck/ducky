pub mod repo_content;

use reqwest::{Client, Error};

use crate::error::Result;

pub struct Requester {
    client: Client,
}

impl Requester {
    pub fn build() -> Result<Self> {
        let client = Client::builder().build()?;
        Ok(Self { client })
    }
}
