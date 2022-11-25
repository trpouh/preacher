use std::{
    error::Error,
    fs::{self, File},
    io::ErrorKind,
    path::{Path, PathBuf},
};

use serde::Deserialize;

use crate::worship::Worship;

/**
 * TODO: Rename
 */
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum FileDestination {
    Simple(String),
    Complex { name: String, create: bool },
}

pub struct FileDeacon<'a> {
    context: &'a FileDestination,
    worship: &'a Worship,
}

impl<'a> FileDeacon<'a> {
    pub fn spawn(context: &'a FileDestination, worship: &'a Worship) -> FileDeacon<'a> {
        FileDeacon { context, worship }
    }

    /**
     * Writes contents to a file. If not specified otherwise, will override.
     */
    pub fn write(&self, contents: &String) -> Result<String, String> {
        let file = self.check_prerequisites();

        match file {
            Ok(path) => fs::write(path, contents),
            Err(err) => todo!()
        };

        Ok(String::new())
    }

    pub fn load(&self) -> Result<String, String> {
        let file = self.check_prerequisites();

        match file {
            Ok(path) => fs::read_to_string(path).map_err(|err| err.to_string()),
            Err(err) => Err(err),
        }
    }

    fn check_prerequisites(&self) -> Result<PathBuf, String> {
        let file = match &self.context {
            FileDestination::Simple(name) => self
                .load_file(name)
                .map_err(|err| err.to_string())
                .map(|_| get_real_path(&self.worship, name)),
            FileDestination::Complex { name, create } => {
                let file = match self.load_file(name) {
                    Ok(file) => Ok(file),
                    Err(err) => {
                        if err.kind() == ErrorKind::NotFound && *create {
                            return fs::File::create(get_real_path(&self.worship, name))
                                .map_err(|err| err.to_string())
                                .map(|_| get_real_path(&self.worship, name));
                        }

                        Err(err.to_string())
                    }
                };

                Err(String::new())
            }
        };

        file
    }

    fn load_file(&self, file: &String) -> Result<File, std::io::Error> {
        File::open(get_real_path(&self.worship, file))
    }
}

fn get_real_path(worship: &Worship, file_name: &str) -> PathBuf {
    Path::new(&worship.run_in_dir).join(&file_name).to_owned()
}
