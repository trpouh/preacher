use crate::psalms::PsalmInfo;

use std::process::Command;
use std::collections::HashMap;
use serde::Deserialize;

use super::{Psalm, PsalmOutput, PsalmVars};


pub struct DebugPsalm { }

#[psalmer::psalm_context]
#[derive(Deserialize)]
pub struct DebugContext {
    message: Option<String>,
    fail: bool
}

impl Psalm<DebugContext> for DebugPsalm {
    fn invoke(context: &DebugContext, _: &crate::worship::Worship, vars: &PsalmVars) -> PsalmOutput {
        
        if context.fail {
            
            let command = Command::new("true").output();

            if let Ok(out) = command {
                println!("out: {}", out.status);
            }

            return PsalmOutput::simple_from_result(context.info.clone(), Ok("ok".to_owned()));

        }
        
        PsalmOutput::simple_from_result(context.info.clone(), Ok("ok".to_owned()))
    }
}