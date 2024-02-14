use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RepoContentFile {
    pub name: String,
    pub path: PathBuf,
    pub sha: String,
    pub size: usize,
    pub url: String,
    pub html_url: String,
    pub git_url: String,
    pub download_url: String,
    pub content: String,
    pub encoding: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "type")]
pub enum RepoContentItem {
    File(RepoContentFile),
    Dir {
        name: String,
        path: PathBuf,
        sha: String,
        url: String,
        html_url: String,
        git_url: String,
    },
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum RepoContent {
    File(RepoContentFile),
    Dir(Vec<RepoContentItem>),
}
