use std::{fs, io::Error, path::PathBuf, process::Command};

pub enum Action {
    RemoveFile(PathBuf),
    WriteFile(PathBuf, Box<[u8]>),
    CreateDir(PathBuf),
    Run(Command),
}

impl Action {
    pub fn execute(self) -> Result<(), Box<Error>> {
        use Action::*;
        match self {
            RemoveFile(path) => fs::remove_file(path)?,
            WriteFile(path, contents) => fs::write(path, contents)?,
            CreateDir(path) => fs::create_dir_all(path)?,
            Run(mut command) => {
                command.output()?;
            }
        }

        Ok(())
    }
}
