use std::path::Path;

use ducky::{
    models::{Repo, Stack},
    requester::Requester, error::Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    let requester = Requester::build()?;

    let stacks: Stack = requester
        .get_repo_content(
            &Repo::new("montmorill", "ducky-stacks"),
            &Path::new("stacks.json"),
            None,
        )
        .await?
        .json()
        .await?;

    println!("{stacks:#?}");

    Ok(())
}
