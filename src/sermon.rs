use serde::Deserialize;
use serde_yaml::from_str;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::psalms::debug::DebugPsalm;
use crate::psalms::file::FilePsalm;
use crate::psalms::hello::HelloPsalm;
use crate::psalms::tz::TzPsalm;
use crate::utils::io::{self, CopyOptions};
use crate::worship::Worship;

use crate::psalms::yaml::YamlPsalm;
use crate::psalms::{Psalm, PsalmOutput, PsalmVars};

use log::debug;

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum PsalmContext {
    Hello(crate::psalms::hello::HelloContext),
    Yaml(crate::psalms::yaml::YamlContext),
    Timezone(crate::psalms::tz::TzContext),
    Debug(crate::psalms::debug::DebugContext),
    File(crate::psalms::file::FileContext),
}

// TODO: macro
fn invoke_psalm(psalm: &PsalmContext, worship: &Worship, vars: &PsalmVars) -> PsalmOutput {
    match psalm {
        PsalmContext::Hello(ctx) => HelloPsalm::invoke(ctx, worship, vars),
        PsalmContext::Yaml(ctx) => YamlPsalm::invoke(ctx, worship, vars),
        PsalmContext::Timezone(ctx) => TzPsalm::invoke(ctx, worship, vars),
        PsalmContext::Debug(ctx) => DebugPsalm::invoke(ctx, worship, vars),
        PsalmContext::File(ctx) => FilePsalm::invoke(ctx, worship, vars),
    }
}

#[derive(Deserialize)]
pub struct Sermon {

    variables: HashMap<String,String>,

    psalms: Vec<PsalmContext>,

    #[serde(skip_deserializing)]
    outputs: Vec<PsalmOutput>,
}

impl Sermon {
    pub fn preach(mut self, worship: &Worship) {

        let vars = &self.variables;

        self.psalms.iter().for_each(move |psalm| {

            let psalm_vars = PsalmVars::new(vars);

            let invocation_output = invoke_psalm(psalm, worship, &psalm_vars);

            let psalm_info = invocation_output.info.clone();

            let id = psalm_info.id.unwrap_or("n/a".to_owned());

            match &invocation_output.result {
                Ok(output) => info!("psalm with id {} was successful: {}", &id, output),
                Err(err) => error!("psalm with id {} was not successful: {}", &id, err),
            };

            self.outputs.push(invocation_output.clone());
        });
    }
}

pub fn initialize(worship: &Worship) -> Result<Sermon, String> {
    //TODO: put copy logic into worship
    let worship_dir = worship.worship_dir.as_str();

    if let Some(repo) = &worship.repo {
        info!("Cloning git repo {} into folder {}", repo, worship_dir);
        io::create_dir(worship_dir, true);
        io::clone_to_dir(repo, worship_dir, worship.branch.as_deref())
    } else {
        let sermon_path = Path::new(&worship.source_folder).join(&worship.sermon);

        //TODO: implement just file checking instead of loading
        if fs::read_to_string(sermon_path).is_ok() {
            debug!(
                "Copying local folder {} into folder {}",
                worship.source_folder,
                worship_dir
            );

            let copy_opts: CopyOptions = CopyOptions {
                source_dir: &worship.source_folder,
                target_dir: worship_dir,
                ensure_target_exists: Some(true),
                exclude: Some([worship_dir, "preacher"].to_vec()),
                without_parent_folder: Some(true),
            };

            io::copy_dir(&copy_opts); //&worship.source_folder, worship_dir);
        } else {

            let error_message = format!(
                "No sermon found under {}/{}",
                &worship.source_folder, &worship.sermon);

            return Err(error_message);
        }
    }

    let sermon_path = Path::new(worship_dir).join(&worship.sermon);

    debug!("Trying to load sermon from path: {}", sermon_path.display());

    fs::read_to_string(sermon_path)
        .map_err(|err| format!("Couldn't load sermon: {}", err))
        .and_then(|c| from_str(&c).map_err(|err| err.to_string()))
}
