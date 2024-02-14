use serde::Deserialize;
use std::{collections::HashMap, path::PathBuf};

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Stack {
    Stack(HashMap<String, Stack>),
    Sheet(PathBuf),
}
