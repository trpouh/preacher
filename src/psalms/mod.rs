use serde::Deserialize;

use crate::worship::Worship;

pub mod deacons;

pub mod yaml;
pub mod hello;
pub mod tz;

#[derive(Clone)]
pub struct PsalmOutput {

    pub info: Option<PsalmInfo>,
    
    pub has_changed: Option<bool>,

    pub result: Result<String,String>

}

/*pub struct InvocationOutput {
    pub message: Result<String,String>
}*/

#[derive(Default, Deserialize, Clone)]
pub struct PsalmInfo {

    pub id: Option<String>,

    pub name: Option<String>,

    pub continue_on_fail: bool
}

impl PsalmOutput {
    /*pub fn failed(info: Option<PsalmInfo>, result: String) -> PsalmOutput {
        PsalmOutput { 
            info, 
            has_changed: None,
            successful: false
        }
    }

    pub fn sucessful(info: Option<PsalmInfo>) -> PsalmOutput {
        PsalmOutput { 
            info,
            has_changed: None,
            successful: true
         }
    }*/

    pub fn simple_from_result(info: Option<PsalmInfo>, result: Result<String,String> ) -> PsalmOutput {
        PsalmOutput { info: info, has_changed: None, result: result }
    }
}

pub trait PsalmInput {
    fn info(&self) -> Option<PsalmInfo>;
}

pub trait Psalm<T> {
    fn invoke(context: &T, worship: &Worship) -> PsalmOutput;
}