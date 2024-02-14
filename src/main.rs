use std::{error::Error, path::Path};

use ducky::{
    models::{Repo, Stack},
    requester::Requester,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
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
