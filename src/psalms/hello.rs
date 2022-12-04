use serde::Deserialize;

use super::{Psalm, PsalmOutput};

//TODO: create hello psalm
pub struct HelloPsalm { }

#[derive(Debug, Deserialize)]
pub struct HelloContext {
    name: Option<String>
}

impl Psalm<HelloContext> for HelloPsalm {
    fn invoke(context: &HelloContext, _: &crate::worship::Worship) -> Result<PsalmOutput, String> {

        let name = context.name.clone();

        println!("hey there {}! congratulations to your first successful worship.", name.unwrap_or_else(||"stranger".to_owned()));
        Ok(PsalmOutput { has_changed: false })
    }
}