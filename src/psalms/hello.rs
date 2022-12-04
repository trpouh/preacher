use crate::psalms::PsalmInfo;
use serde::Deserialize;

use super::{Psalm, PsalmOutput};


pub struct HelloPsalm { }

#[psalmer::psalm_context]
#[derive(Debug, Deserialize)]
pub struct HelloContext {
    name: Option<String>
}

#[psalmer::psalm]
struct Test {}

impl Psalm<HelloContext> for HelloPsalm {
    fn invoke(context: &HelloContext, _: &crate::worship::Worship) -> Result<PsalmOutput, String> {
        
        let name = context.name.clone();
        let info = context.info.clone().unwrap();

        println!("running psalm with id: {}", info.id.unwrap_or("unknown".to_owned()));

        println!("hey there {}! congratulations to your first successful worship.", name.unwrap_or_else(||"stranger".to_owned()));
        Ok(PsalmOutput { has_changed: false })
    }
}