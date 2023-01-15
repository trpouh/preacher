use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};
use uuid::Uuid;

use crate::{psalms::PsalmVars, worship::Worship};
use serde::Deserialize;

use super::template::Templating;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Source {
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
    Inline {
        content: String,
    },
}

#[derive(Debug, Deserialize)]
pub struct FileSource {
    #[serde(flatten)]
    source: Source,

    template: Option<Templating>,
}

impl FileSource {
    pub fn to_deacon<'a>(
        &'a self,
        worship: &'a Worship,
        vars: &PsalmVars,
    ) -> Result<FileSourceDeacon, String> {
        FileSourceDeacon::new(self, worship, vars)
    }
}
pub struct FileSourceDeacon {
    path: PathBuf,
}

impl FileSourceDeacon {
    fn new<'a>(
        source: &'a FileSource,
        worship: &'a Worship,
        vars: &PsalmVars,
    ) -> Result<FileSourceDeacon, String> {
        //TODO: clean a bit up
        let path: Result<PathBuf, String> = match &source.source {
            Source::Http { url: _ } => todo!("should download file to local disk"),
            Source::Git {
                repo: _,
                branch: _,
                path: _,
            } => todo!("should download file to local disk"),
            Source::Simple(path) => Ok(Path::new(&worship.target_folder).join(path)),
            Source::Complex { path, in_worship } => {
                let root = if in_worship.unwrap_or(false) {
                    &worship.worship_dir
                } else {
                    &worship.target_folder
                };
                Ok(Path::new(&root).join(path))
            }
            Source::Inline { content } => {
                FileSourceDeacon::write_to_tmp(&worship.worship_dir, content)
            }
        };

        let templated = path.and_then(|path| {
            if let Some(template) = &source.template {
                return template
                    .to_deacon(&path)
                    .template(vars.vars)
                    .and_then(|res| FileSourceDeacon::write_to_tmp(&worship.worship_dir, &res));
            }

            Ok(path)
        });

        templated.map(|path| FileSourceDeacon { path })
    }

    fn write_to_tmp(path: &String, content: &String) -> Result<PathBuf, String> {
        let tmp_file = Uuid::new_v4().to_string();
        let target = Path::new(path).join(tmp_file);

        let file = File::create(target.clone());

        file.and_then(|mut f| f.write_all(content.as_bytes()))
            .map_err(|err| err.to_string())
            .map(|_| target)
    }

    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }

    pub fn file_content(&self) -> Result<String, String> {
        let file = File::open(&self.path);

        file.and_then(|mut file| {
            let mut contents = String::new();
            file.read_to_string(&mut contents).map(|_| contents)
        })
        .map_err(|err| err.to_string())
    }
}

#[cfg(test)]
mod tests {

    mod path {
        use std::{collections::HashMap, path::Path};

        use crate::{
            psalms::{
                deacons::file::{
                    source::{FileSource, Source},
                    template::Templating,
                },
                PsalmVars,
            },
            worship::Worship,
        };
        #[test]
        fn test_simple() {
            let source = FileSource {
                source: Source::Simple("file.txt".to_owned()),
                template: None,
            };

            let worship = Worship {
                branch: None,
                repo: None,
                sermon: "".to_owned(),
                worship_dir: "".to_owned(),
                source_folder: "".to_owned(),
                target_folder: "/test".to_owned(),
            };

            let under_test = source.to_deacon(
                &worship,
                &PsalmVars {
                    vars: &HashMap::default(),
                },
            );

            let expected = Path::new("/test").join("file.txt");

            assert_eq!(&expected, under_test.unwrap().get_path())
        }

        #[test]
        fn test_complex() {
            let source_local = FileSource {
                source: Source::Complex {
                    path: "file.txt".to_owned(),
                    in_worship: Some(false),
                },
                template: None,
            };

            let source_worship = FileSource {
                source: Source::Complex {
                    path: "file.txt".to_owned(),
                    in_worship: Some(true),
                },
                template: None,
            };

            let worship = Worship {
                branch: None,
                repo: None,
                sermon: "".to_owned(),
                worship_dir: "/worship".to_owned(),
                source_folder: "".to_owned(),
                target_folder: "/target".to_owned(),
            };

            let under_test_local = source_local.to_deacon(
                &worship,
                &PsalmVars {
                    vars: &HashMap::default(),
                },
            );
            let under_test_worship = source_worship.to_deacon(
                &worship,
                &PsalmVars {
                    vars: &HashMap::default(),
                },
            );

            let expected_local = Path::new("/target").join("file.txt");
            let expected_worship = Path::new("/worship").join("file.txt");

            assert_eq!(&expected_local, under_test_local.unwrap().get_path());
            assert_eq!(&expected_worship, under_test_worship.unwrap().get_path());
        }

        #[test]
        fn test_inline_template() {
            let source = FileSource {
                source: Source::Inline {
                    content: "{% set greeting = \"Hello\" %}{{ greeting }}".to_owned(),
                },
                template: Some(Templating {
                    flavor: crate::psalms::deacons::file::template::TemplatingLanguage::J2,
                }),
            };

            let worship = Worship {
                branch: None,
                repo: None,
                sermon: "".to_owned(),
                worship_dir: "/worship".to_owned(),
                source_folder: "".to_owned(),
                target_folder: "/target".to_owned(),
            };

            let under_test_local = source.to_deacon(
                &worship,
                &PsalmVars {
                    vars: &HashMap::default(),
                },
            );
            let expected_local = Path::new("/target").join("file.txt");

            assert_eq!(&expected_local, under_test_local.unwrap().get_path());
        }
    }
}
