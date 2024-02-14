pub mod repo_content;

use reqwest::{Client, Error};

pub struct Requester {
    client: Client,
}

impl Requester {
    pub fn build() -> Result<Self, Error> {
        let client = Client::builder().build()?;
        Ok(Self { client })
    }
}
