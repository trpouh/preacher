use serde::Deserialize;

use super::Psalm;

//TODO: create hello psalm
pub struct HelloPsalm { }

#[derive(Debug, Deserialize)]
pub struct HelloContext {
    name: Option<String>
}

impl Psalm<HelloContext> for HelloPsalm {
    fn invoke(context: &HelloContext, worship: &crate::worship::Worship) -> Result<String,String> {

        let name = context.name.clone();

        println!("hey there {}! congratulations to your first successful worship.", name.unwrap_or_else(||"stranger".to_owned()));
        Ok(String::new())
    }
}