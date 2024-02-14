use super::Requester;
use crate::models::{Repo, RepoContent};
use reqwest::{Response, Result};
use std::path::Path;

impl Requester {
    pub async fn get_repo_content(
        &self,
        repo: &Repo,
        path: &Path,
        ref_: Option<&str>,
    ) -> Result<Response> {
        let mut url = format!(
            "https://api.github.com/repos/{}/contents/{}",
            repo.to_string(),
            path.to_str().unwrap()
        );

        if let Some(ref_) = ref_ {
            url += format!("&ref={ref_}").as_str();
        }
        let resp = self
            .client
            .get(url.clone())
            .header("user-agent", "ducky")
            .send()
            .await?;

        let resp: RepoContent = resp.json().await?;

        let resp = if let RepoContent::File(file) = resp {
            reqwest::get(file.download_url).await?
        } else {
            todo!("Directory is not supported.")
        };

        Ok(resp)
    }
}
