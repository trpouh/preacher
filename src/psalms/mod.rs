use serde::Deserialize;

pub mod yaml;

pub trait Psalm<T> {
    fn invoke(context: &T) -> Result<String,String>;
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum FileDestination {

    Simple(String),
    Complex {
        name: String,
        create: bool
    }
}