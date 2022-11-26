use serde::Deserialize;
use serde_yaml::from_str;
use std::fmt::Debug;
use std::fs;
use std::path::Path;

use crate::worship::Worship;

use crate::psalms::Psalm;
use crate::psalms::yaml::YamlPsalm;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum PsalmContext {
    yaml(crate::psalms::yaml::YamlContext),
}

fn invoke_psalm(psalm: &PsalmContext, worship: &Worship) -> Result<String,String> {
    match (psalm) {
        PsalmContext::yaml(ctx) => YamlPsalm::invoke(ctx, &worship)
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

    let sermon_path = Path::new(&worship.run_in_dir)
        .join(&worship.sermon)
        .to_owned();

    fs::read_to_string(sermon_path)
        .map_err(|err| err.to_string())
        .and_then(|c| from_str(&c).map_err(|err| err.to_string()))
}