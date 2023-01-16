use serde_yaml::from_str;

use crate::{psalms::prelude::{core::*}, sermon::{Sermon, Status, SermonStatus}};

use super::deacons::prelude::FileSource;

/*

Use this file as a starting point for a new psalm.
Strg+F "Psalms" and replace with desired name

*/

pub struct SermonPsalm {}
#[psalmer::psalm_context]
#[derive(Deserialize)]
pub struct SermonContext {
    sermon: FileSource
}

//TODO: combine both psalm and sermon vars
impl Psalm<SermonContext> for SermonPsalm {
    fn invoke(context: &SermonContext, worship: &crate::worship::Worship, vars: &PsalmVars) -> PsalmOutput {

        let deacon = context.sermon.to_deacon(worship, vars);

        let sermon: Result<Sermon,String> = deacon.and_then(|x| x.file_content()).and_then(|c|from_str(&c).map_err(|e|e.to_string()));

        match sermon {
            Ok(s) => SermonPsalm::from_sermon_status(&s.preach_with_vars(worship, vars.get_map()), context.info.clone()),
            Err(err) => PsalmOutput::failed(context.info.clone(), err)
        }
    }
}

impl SermonPsalm {
    fn from_sermon_status(status: &Status, info: PsalmInfo) -> PsalmOutput {
        match status {
            SermonStatus::OK => PsalmOutput::sucessful(info)
        }
    }
}