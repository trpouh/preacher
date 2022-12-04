use serde::Deserialize;
use serde_yaml::from_str;
use std::fmt::Debug;
use std::fs;
use std::path::Path;

use crate::lib::io::{self, CopyOptions};
use crate::psalms::hello::HelloPsalm;
use crate::worship::Worship;

use crate::psalms::Psalm;
use crate::psalms::yaml::YamlPsalm;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum PsalmContext {
    Yaml(crate::psalms::yaml::YamlContext),
    Hello(crate::psalms::hello::HelloContext)
}

fn invoke_psalm(psalm: &PsalmContext, worship: &Worship) -> Result<String,String> {
    match psalm {
        PsalmContext::Yaml(ctx) => YamlPsalm::invoke(ctx, &worship),
        PsalmContext::Hello(ctx) => HelloPsalm::invoke(ctx, worship),
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

    let tmp_dir = worship.tmp_dir.as_str();

    if let Some(repo) = &worship.repo {
        println!("Cloning git repo {} into folder {}", repo, tmp_dir);
        io::create_dir(&tmp_dir, true);
        io::clone_to_dir(repo, tmp_dir, worship.branch.as_ref().map(|x| &**x))
        
    } else {

        let sermon_path = Path::new( &worship.source_folder )
        .join(&worship.sermon)
        .to_owned();

        //TODO: implement just file checking instead of loading
        //TODO: same directory creates recursion
        if let Ok(_) = fs::read_to_string(sermon_path) {
            println!("Copying local folder {} into folder {}", worship.source_folder, tmp_dir);
            
            let copy_opts: CopyOptions = CopyOptions {
                source_dir: &worship.source_folder,
                target_dir: tmp_dir,
                ensure_target_exists: Some(true),
                exclude: Some([tmp_dir].to_vec()),
                without_parent_folder: Some(true)
            };
            
            io::copy_dir(&copy_opts);//&worship.source_folder, tmp_dir);

        } else {
            println!("Couldn't find sermon {} in local folder: {}", &worship.sermon, &worship.source_folder);
            return Err("No sermon found".to_string());
        }
    }

    let sermon_path = Path::new(tmp_dir)
        .join(&worship.sermon)
        .to_owned();

    println!("Trying to load sermon from path: {}", sermon_path.display());

    fs::read_to_string(sermon_path)
        .map_err(|err| format!("Couldn't load sermon: {}", err.to_string()))
        .and_then(|c| from_str(&c).map_err(|err| err.to_string()))
}