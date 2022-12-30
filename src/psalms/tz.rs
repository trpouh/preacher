use crate::psalms::prelude::{core::*};
use std::process::Command;

use crate::{utils::cmd::spawn_and_map_to_res};
pub struct TzPsalm {}

#[psalmer::psalm_context]
#[derive(Deserialize)]
pub struct TzContext {
    tz: String,
}

/**
 * Psalm for chaning the timezone on a linux based host.
 */
impl TzPsalm {
    fn set_timezone(tz: &str) -> Result<String, String> {
        let mut command = Command::new("timedatectl");
        command.args(["set-timezone", tz, "--no-ask-password"]);

        spawn_and_map_to_res(&mut command)
    }
}

impl Psalm<TzContext> for TzPsalm {
    fn invoke(context: &TzContext, _: &crate::worship::Worship, vars: &PsalmVars) -> PsalmOutput {
        let tz = TzPsalm::set_timezone(&context.tz);

        PsalmOutput::simple_from_result(context.info.clone(), tz)
    }
}
