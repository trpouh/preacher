use crate::worship::Worship;

pub mod yaml;
pub mod deacons;
pub mod hello;

pub trait Psalm<T> {
    fn invoke(context: &T, worship: &Worship) -> Result<String,String>;
}