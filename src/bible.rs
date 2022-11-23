use serde::Deserialize;
use serde_yaml::from_str;
use std::fmt::Debug;
use std::fs;
use std::iter::Enumerate;
use std::path::Path;

use crate::config::Invocation;
use crate::psalms::Psalm;
use crate::psalms::yaml::YamlPsalm;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum PsalmContext {
    yaml(crate::psalms::yaml::YamlContext),
}

fn invoke_psalm(psalm: &PsalmContext) -> Result<String,String> {
    match (psalm) {
        PsalmContext::yaml(ctx) => YamlPsalm::invoke(ctx)
    }
}


#[derive(Debug, Deserialize)]
pub struct Blurb {
    version: String,
}

#[derive(Debug, Deserialize)]
pub struct Bible {
    blurb: Blurb,
    psalms: Vec<PsalmContext>,
}

impl Bible {

    pub fn preach(&self) {

        self.psalms.iter().for_each(|psalm| {
            
            let res = invoke_psalm(&psalm);
            print!("was ok: {}", res.is_ok())

        });
    }
}

pub fn initialize(invocation: Invocation) -> Result<Bible, String> {
    let sermon_path = Path::new(&invocation.run_in_dir)
        .join(&invocation.sermon)
        .to_owned();

    fs::read_to_string(sermon_path)
        .map_err(|err| err.to_string())
        .and_then(|c| from_str(&c).map_err(|err| err.to_string()))
}