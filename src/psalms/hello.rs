use crate::psalms::prelude::{core::*};

pub struct HelloPsalm {}
#[psalmer::psalm_context]
#[derive(Deserialize)]
pub struct HelloContext {
    name: Option<String>
}

impl Psalm<HelloContext> for HelloPsalm {
    fn invoke(context: &HelloContext, _: &crate::worship::Worship, _: &PsalmVars) -> PsalmOutput {
        
        let name = context.name.clone();

        info!("Hey there {}! Congratulations to your first successful worship.", name.unwrap_or_else(||"stranger".to_owned()));
        
        PsalmOutput::simple_from_result(context.info.clone(), Ok("ok".to_owned()))
    }
}