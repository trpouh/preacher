use std::path::{Path, PathBuf};

use crate::worship::Worship;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum FileSource {
    Simple(String),
    Complex {
        path: String,
        in_worship: Option<bool>, //TODO: can't come up with a good name
    },
    Http {
        url: String,
    },
    Git {
        repo: String,
        path: String,
        branch: Option<bool>,
    },
}

impl FileSource {
    pub fn to_deacon<'a>(&'a self, worship: &'a Worship) -> Result<FileSourceDeacon, String> {
        FileSourceDeacon::new(self, worship)
    }
}
pub struct FileSourceDeacon {
    path: PathBuf,
}

impl FileSourceDeacon {
    fn new<'a>(source: &'a FileSource, worship: &'a Worship) -> Result<FileSourceDeacon, String> {
        //TODO: clean a bit up
        let path: Result<PathBuf, String> = match source {
            FileSource::Http { url: _ } => todo!("should download file to local disk"),
            FileSource::Git {
                repo: _,
                branch: _,
                path: _,
            } => todo!("should download file to local disk"),
            FileSource::Simple(path) => Ok(Path::new(&worship.target_folder).join(path)),
            FileSource::Complex { path, in_worship } => {
                let root = if in_worship.unwrap_or(false) {
                    &worship.worship_dir
                } else {
                    &worship.target_folder
                };
                Ok(Path::new(&root).join(path))
            }
        };

        path.map(|_path| FileSourceDeacon { path: _path })
    }

    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }
}

#[cfg(test)]
mod tests {

    mod path {
        use std::path::Path;

        use crate::{psalms::deacons::file::source::FileSource, worship::Worship};

        #[test]
        fn test_simple() {
            let source = FileSource::Simple("file.txt".to_owned());

            let worship = Worship {
                branch: None,
                repo: None,
                sermon: "".to_owned(),
                worship_dir: "".to_owned(),
                source_folder: "".to_owned(),
                target_folder: "/test".to_owned(),
            };

            let under_test = source.to_deacon(&worship);

            let expected = Path::new("/test").join("file.txt");

            assert_eq!(&expected, under_test.unwrap().get_path())
        }

        #[test]
        fn test_complex() {
            let source_local = FileSource::Complex {
                path: "file.txt".to_owned(),
                in_worship: Some(false),
            };

            let source_worship = FileSource::Complex {
                path: "file.txt".to_owned(),
                in_worship: Some(true),
            };

            let worship = Worship {
                branch: None,
                repo: None,
                sermon: "".to_owned(),
                worship_dir: "/worship".to_owned(),
                source_folder: "".to_owned(),
                target_folder: "/target".to_owned(),
            };

            let under_test_local = source_local.to_deacon(&worship);
            let under_test_worship = source_worship.to_deacon(&worship);

            let expected_local = Path::new("/target").join("file.txt");
            let expected_worship = Path::new("/worship").join("file.txt");

            assert_eq!(&expected_local, under_test_local.unwrap().get_path());
            assert_eq!(&expected_worship, under_test_worship.unwrap().get_path());
        }
    }
}
