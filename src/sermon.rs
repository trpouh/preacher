use serde::Deserialize;
use serde_yaml::from_str;
use std::error::Error;
use std::fmt::Debug;
use std::fs;
use std::path::Path;
use std::process::Command;

use crate::lib::io;
use crate::worship::{Worship, self};

use crate::psalms::Psalm;
use crate::psalms::yaml::YamlPsalm;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum PsalmContext {
    Yaml(crate::psalms::yaml::YamlContext),
}

fn invoke_psalm(psalm: &PsalmContext, worship: &Worship) -> Result<String,String> {
    match (psalm) {
        PsalmContext::Yaml(ctx) => YamlPsalm::invoke(ctx, &worship)
    }
}

#[derive(Debug, Deserialize)]
pub struct Sermon {
    psalms: Vec<PsalmContext>,
}

impl Sermon {

    pub fn preach(&self, worship: &Worship) {

        self.psalms.iter().for_each(|psalm| {
            
            let res = invoke_psalm(&psalm, worship);
            print!("was ok: {}", res.is_ok())

        });
    }
}

pub fn initialize(worship: &Worship) -> Result<Sermon, String> {

    let tmp_dir = &worship.tmp_dir;

    if let Some(repo) = &worship.repo {
        println!("Cloning git repo {} into folder {}", repo, tmp_dir);
        io::create_dir(&tmp_dir, true);
        io::clone_to_dir(repo, tmp_dir, worship.branch.as_ref().map(|x| &**x))
    } else if let Some(source_dir) = &worship.source_folder {
        println!("Copying local folder {} into folder {}", source_dir, tmp_dir);
        io::create_dir(&tmp_dir, true);
        io::copy_dir(source_dir, tmp_dir);
    } else {
        return Err("No location for sermon found. Either provide a git repo via --repo or a local folder via --source-folder".to_owned());
    }

    let sermon_path = Path::new(tmp_dir)
        .join(&worship.sermon)
        .to_owned();

    println!("Trying to load sermon from path: {}", sermon_path.display());

    fs::read_to_string(sermon_path)
        .map_err(|err| format!("Couldn't load sermon: {}", err.to_string()))
        .and_then(|c| from_str(&c).map_err(|err| err.to_string()))
}