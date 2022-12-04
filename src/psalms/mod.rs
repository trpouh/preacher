use crate::worship::Worship;

pub mod yaml;
pub mod deacons;
pub mod hello;

pub struct PsalmOutput {
    
    pub has_changed: bool

}

pub struct PsalmInfo<'a> {
    pub id: Option<&'a str>,
    pub name: Option<&'a str>
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