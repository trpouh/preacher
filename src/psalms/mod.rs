use serde::Deserialize;

use crate::worship::Worship;

pub mod yaml;
pub mod deacon;

pub trait Psalm<T> {
    fn invoke(context: &T, worship: &Worship) -> Result<String,String>;
}