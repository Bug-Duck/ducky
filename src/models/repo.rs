use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Repo {
    pub owner: String,
    pub repo: String,
}

impl Repo {
    pub fn new(owner: &str, repo: &str) -> Self {
        Self {
            owner: String::from(owner),
            repo: String::from(repo),
        }
    }
}

impl ToString for Repo {
    fn to_string(&self) -> String {
        format!("{}/{}", self.owner, self.repo)
    }
}
