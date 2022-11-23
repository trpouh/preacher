use serde::Deserialize;

use crate::Psalm;

use super::FileDestination;

#[derive(Debug, Deserialize)]
pub struct YamlContext {
    file: FileDestination,
    path: String,
    r#override: String
}

pub struct YamlPsalm {}

impl Psalm<YamlContext> for YamlPsalm {

    fn invoke(context: &YamlContext) -> Result<String,String> {
        
        print!("yaml: {:#?}", context);

        Ok("OK".to_owned())
    }
}