pub use std::{collections::HashMap, iter::Map};

use serde::Deserialize;

use crate::worship::Worship;

pub mod deacons;

pub mod debug;
pub mod file;
pub mod hello;
pub mod tz;
pub mod yaml;
pub mod sermon;

pub mod prelude {
    pub mod core {
        
        pub use std::collections::HashMap;

        pub use crate::{
            psalms::{Psalm, PsalmInput, PsalmOutput, PsalmInfo, PsalmVars},
            worship::Worship,
        };
        
        pub use serde::Deserialize;
    }

    pub mod deacons {
        pub use crate::psalms::deacons::prelude::*;
    }
}

#[derive(Clone)]
pub struct PsalmOutput {
    pub info: PsalmInfo,

    pub has_changed: Option<bool>,

    pub result: Result<String, String>,
}

#[derive(Default, Deserialize, Clone, Debug)]
pub struct PsalmInfo {
    pub id: Option<String>,

    pub name: Option<String>,

    pub continue_on_fail: Option<bool>,
}

pub struct PsalmVars<'a> {
    vars: &'a HashMap<String, String>,
}

impl<'a> PsalmVars<'a> {

    pub fn new(vars: &'a HashMap<String,String>) -> PsalmVars<'a> {
        PsalmVars {
            vars
        }
    }

    fn get_map(&self) -> &HashMap<String,String> {
        self.vars
    }
}

impl PsalmOutput {
    pub fn failed(info: PsalmInfo, err: String) -> PsalmOutput {
        PsalmOutput {
            info,
            has_changed: None,
            result: Err(err),
        }
    }

    
    pub fn sucessful(info: PsalmInfo) -> PsalmOutput {
        PsalmOutput {
            info,
            has_changed: None,
            result: Ok("OK".to_owned())
         }
    }

    pub fn simple_from_result(info: PsalmInfo, result: Result<String, String>) -> PsalmOutput {
        PsalmOutput {
            info,
            has_changed: None,
            result,
        }
    }
}

pub trait PsalmInput {
    fn info(&self) -> Option<PsalmInfo>;
}

pub trait Psalm<T> {
    fn invoke(context: &T, worship: &Worship, vars: &PsalmVars) -> PsalmOutput;
}