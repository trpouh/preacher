use crate::Psalm;

pub struct YamlContext {

}

pub struct YamlPsalm {

}

impl Psalm<YamlContext> for YamlPsalm {

    fn invoke(&self, context: YamlContext) -> Result<String,String> {
        println!("invoked. yes!");
        Ok("OK".to_owned())
    }

}