use serde::Deserialize;

use crate::worship::Worship;

pub mod deacons;

pub mod yaml;
pub mod hello;
pub mod tz;
pub mod debug;
pub mod file;

#[derive(Clone)]
pub struct PsalmOutput {

    pub info: PsalmInfo,
    
    pub has_changed: Option<bool>,

    pub result: Result<String,String>

}

#[derive(Default, Deserialize, Clone, Debug)]
pub struct PsalmInfo {

    pub id: Option<String>,

    pub name: Option<String>,

    pub continue_on_fail: Option<bool>
}

impl PsalmOutput {

    pub fn failed(info: PsalmInfo, err: String) -> PsalmOutput {
        PsalmOutput { 
            info, 
            has_changed: None,
            result: Err(err)
        }
    }

    /*
    pub fn sucessful(info: Option<PsalmInfo>) -> PsalmOutput {
        PsalmOutput { 
            info,
            has_changed: None,
            successful: true
         }
    }*/

    pub fn simple_from_result(info: PsalmInfo, result: Result<String,String> ) -> PsalmOutput {
        PsalmOutput { info, has_changed: None, result: result }
    }
}

pub trait PsalmInput {
    fn info(&self) -> Option<PsalmInfo>;
}

pub trait Psalm<T> {
    fn invoke(context: &T, worship: &Worship) -> PsalmOutput;
}