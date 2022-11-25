use std::fs::File;

use serde::Deserialize;

use crate::{Psalm, psalms::deacon::FileDeacon, worship::Worship};

use super::deacon::FileDestination;

#[cfg(test)]
mod tests{

    #[test]
    fn replace_content() {
        assert_eq!(1, 1)
    }
}

#[derive(Debug, Deserialize)]
pub struct YamlContext {
    file: FileDestination,
    path: String,
    r#override: String
}

pub struct YamlPsalm {}

impl Psalm<YamlContext> for YamlPsalm {

    fn invoke(context: &YamlContext, worship: &Worship) -> Result<String,String> {

        let file_deacon = FileDeacon::spawn(&context.file, &worship);

        let contents = file_deacon.load()?;

        file_deacon.write(&context.r#override);
        
        println!("yaml: {:#?}", contents);

        Ok("OK".to_owned())
    }
}