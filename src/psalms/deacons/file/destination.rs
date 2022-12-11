use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::worship::Worship;
use serde::Deserialize;

/**
 * TODO: Rename
 */
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum FileDestination {
    Simple(String),
    Complex {
        path: String,
        create_parents: Option<bool>,
        create: Option<bool>,
    },
}

impl FileDestination {
    pub fn to_deacon<'a>(&'a self, worship: &'a Worship) -> Result<FileDeacon, String> {
        FileDeacon::new(self, worship)
    }
}

pub struct FileDeacon {
    path: PathBuf,
}

impl<'a> FileDeacon {
    pub fn new(
        destination: &'a FileDestination,
        worship: &'a Worship,
    ) -> Result<FileDeacon, String> {
        // TODO: result needed?
        // TODO: implement checking and parent creating
        let target_path_if_exists = |path: &String| {
            let target = Path::new(&worship.target_folder).join(path);

            if target.exists() {
                return Ok(target);
            }

            Err(format!("no such file: '{}'", target.display()))
        };

        let path: Result<PathBuf, String> = match destination {
            FileDestination::Simple(path) => target_path_if_exists(path),
            FileDestination::Complex {
                path,
                create,
                create_parents,
            } => {
                let should_not_create = !create.unwrap_or(false);

                let should_create_parents = create_parents.unwrap_or(false);

                let t = match target_path_if_exists(path) {
                    Ok(path) => Ok(path),
                    Err(err) => {
                        if should_not_create {
                            return Err(err);
                        }

                        let target = Path::new(&worship.target_folder).join(path);

                        if should_create_parents
                            && target.parent().map(|p| !p.exists()).unwrap_or(false)
                        {
                            //TODO: use -- but how?
                            target.parent().and_then(|parent| {
                                fs::create_dir_all(parent)
                                    .map_err(|err| err.to_string())
                                    .ok()
                            });
                        };

                        fs::File::create(target.clone())
                            .map(|_| target)
                            .map_err(|err| err.to_string())
                    }
                };

                t
            }
        };

        path.map(|path| FileDeacon { path })
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    /**
     * Writes contents to a file. If not specified otherwise, will override.
     */
    pub fn write(&self, contents: &String) -> Result<String, String> {
        fs::write(self.path(), contents)
            .map_err(|err| err.to_string())
            .map(|_| "OK".to_owned())
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, path::Path};

    use crate::worship::Worship;

    use super::FileDestination;

    fn default_worship() -> Worship {
        Worship {
            repo: None,
            branch: None,
            source_folder: ".".to_owned(),
            sermon: "".to_owned(),
            worship_dir: "".to_owned(),
            target_folder: ".".to_owned(),
        }
    }

    #[test]
    fn test_simple_file_loading() {
        let worship = default_worship();

        let file = "file2.txt".to_owned();

        File::create(&file).unwrap();

        let deacon = FileDestination::Simple(file.clone()).to_deacon(&worship);

        assert_eq!(deacon.is_ok(), true);
        assert_eq!(deacon.unwrap().path(), &Path::new(".").join(&file));

        std::fs::remove_file(&file).unwrap();
    }

    #[test]
    fn test_file_creating_without_parent() {
        let worship = default_worship();

        let file_without_parents = "file.txt".to_owned();

        let deacon_create_no_parents = FileDestination::Complex {
            path: file_without_parents.clone(),
            create: Some(true),
            create_parents: Some(false),
        }
        .to_deacon(&worship);

        assert_eq!(deacon_create_no_parents.is_ok(), true);
        assert_eq!(
            deacon_create_no_parents.unwrap().path(),
            &Path::new(".").join("file.txt")
        );
        std::fs::remove_file(&file_without_parents).unwrap();
    }

    #[test]
    fn test_file_creating_with_parent_should_fail() {
        let worship = default_worship();

        let file_with_parents = "test/file.txt".to_owned();

        let deacon_create_without_parents = FileDestination::Complex {
            path: file_with_parents.clone(),
            create: Some(true),
            create_parents: Some(false),
        }
        .to_deacon(&worship);

        assert_eq!(deacon_create_without_parents.is_ok(), false);
    }

    #[test]
    fn test_file_creating_with_parent() {
        let worship = default_worship();

        let file_with_parents = "testtmp/file2.txt".to_owned();

        let deacon_create_with_parents = FileDestination::Complex {
            path: file_with_parents.clone(),
            create: Some(true),
            create_parents: Some(true),
        }
        .to_deacon(&worship);

        assert_eq!(deacon_create_with_parents.is_ok(), true);
        assert_eq!(
            deacon_create_with_parents.unwrap().path(),
            &Path::new(".").join("testtmp").join("file2.txt")
        );
        std::fs::remove_file(&file_with_parents).unwrap();
        std::fs::remove_dir("testtmp").unwrap();
    }

    #[test]
    fn test_complex_is_ok() {
        let worship = default_worship();

        let file = "file3.txt".to_owned();

        File::create(&file).unwrap();

        let deacon = FileDestination::Complex {
            path: file.clone(),
            create: Some(false),
            create_parents: Some(false),
        }
        .to_deacon(&worship);

        assert_eq!(deacon.is_ok(), true);
        assert_eq!(deacon.unwrap().path(), &Path::new(".").join(&file));
        std::fs::remove_file(&file).unwrap();
    }
}
