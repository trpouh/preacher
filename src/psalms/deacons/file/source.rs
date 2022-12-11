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
        git_url: String,
        branch: Option<bool>,
    },
}

impl FileSource {
    pub fn to_deacon<'a>(&'a self, worship: &'a Worship) -> FileSourceDeacon {
        FileSourceDeacon::new(self, worship)
    }
}
pub struct FileSourceDeacon<'a> {
    path: PathBuf,
    worship: &'a Worship, //TODO: maybe remove? doesn't seem necessary
}

impl<'a> FileSourceDeacon<'a> {
    fn new(source: &'a FileSource, worship: &'a Worship) -> FileSourceDeacon<'a> {
        //TODO: clean a bit up
        let path = match source {
            FileSource::Http { url: _ } => todo!("should download file to local disk"),
            FileSource::Git {
                git_url: _,
                branch: _,
            } => todo!("should download file to local disk"),
            FileSource::Simple(path) => Path::new(&worship.target_folder).join(path),
            FileSource::Complex { path, in_worship } => {
                let root = if in_worship.unwrap_or(false) {
                    &worship.worship_dir
                } else {
                    &worship.target_folder
                };
                Path::new(&root).join(path)
            }
        };

        let instance = FileSourceDeacon {
            worship: worship,
            path,
        };

        instance
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

            assert_eq!(&expected, under_test.get_path())
        }

        #[test]
        fn test_complex() {
            let source_local = FileSource::Complex {
                path: "file.txt".to_owned(),
                in_worship: Some(false),
            };

            let source_worship = FileSource::Complex {
                path: "file.txt".to_owned(),
                in_worship: Some(true)
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

            assert_eq!(&expected_local, under_test_local.get_path());
            assert_eq!(&expected_worship, under_test_worship.get_path());
        }
    }
}
