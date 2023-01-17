use crate::psalms::PsalmInfo;

use std::collections::HashMap;
use serde::Deserialize;

use super::{Psalm, PsalmOutput, PsalmVars, deacons::prelude::FileSource};


pub struct DebugPsalm { }

#[psalmer::psalm_context]
#[derive(Deserialize)]
pub struct DebugContext {
    echo: FileSource
}

impl Psalm<DebugContext> for DebugPsalm {
    fn invoke(context: &DebugContext, worship: &crate::worship::Worship, vars: &PsalmVars) -> PsalmOutput {
        
        let deacon = context.echo.to_deacon(worship, vars).and_then(|d|d.file_content());

        if let Ok(c) = deacon {
             info!("---Debugging Start");
             info!("{}", c);
             info!("---Debugging End");
        } else if let Err(e) = deacon {
            error!("Error debugging: {}", e);
        }

        PsalmOutput::simple_from_result(context.info.clone(), Ok("ok".to_owned()))
    }
}