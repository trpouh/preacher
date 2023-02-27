use crate::psalms::prelude::{core::*};

use super::deacons::prelude::Source;

/*

Use this file as a starting point for a new psalm.
Strg+F "Exec" and replace with desired name

*/

pub struct ExecPsalm {}
#[psalmer::psalm_context]
#[derive(Deserialize)]
pub struct ExecContext {
    source: Source,
    executable: String
}

impl Psalm<ExecContext> for ExecPsalm {
    fn invoke(context: &ExecContext, _: &crate::worship::Worship, _: &PsalmVars) -> PsalmOutput {
        
        PsalmOutput::simple_from_result(context.info.clone(), Ok("ok".to_owned()))
    }
}