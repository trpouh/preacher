use serde::Deserialize;

use crate::worship::Worship;

pub mod yaml;
pub mod deacons;
pub mod hello;

pub struct PsalmOutput {
    
    pub has_changed: bool

}

#[derive(Debug, Deserialize, Clone)]
pub struct PsalmInfo {

    pub id: Option<String>,

    pub name: Option<String>
}

impl PsalmOutput {
    pub fn empty() -> PsalmOutput {
        PsalmOutput { has_changed: false }
    }
}

pub trait PsalmInput {
    fn info(&self) -> Option<PsalmInfo>;
}

pub trait Psalm<T> {
    fn invoke(context: &T, worship: &Worship) -> Result<PsalmOutput,String>;
}