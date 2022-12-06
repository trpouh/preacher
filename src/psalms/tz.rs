use std::process::Command;

use crate::psalms::PsalmInfo;
use serde::Deserialize;

use super::{Psalm, PsalmOutput};
pub struct TzPsalm { }

#[psalmer::psalm_context]
#[derive(Deserialize)]
pub struct TzContext {
    tz: String
}


/**
 * Psalm for chaning the timezone on a linux based host.
 */
impl TzPsalm {
    fn set_timezone(tz: &str) -> Result<String, String> {

        let mut command = Command::new("timedatectl");
        command.args(["set-timezone", tz, "--no-ask-password"]);

        let child = command.spawn().and_then(|c| {

            c.wait_with_output()
                .map(|out| String::from_utf8(out.stdout).expect("command successful; no output"))


        }).map_err(|err|{
            err.to_string()
        });

        child
    }
}

impl Psalm<TzContext> for TzPsalm {
    fn invoke(context: &TzContext, _: &crate::worship::Worship) -> PsalmOutput {
    
        let tz = TzPsalm::set_timezone(&context.tz);
        
        PsalmOutput::simple_from_result(context.info.clone(), tz)
    }
}