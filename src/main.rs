#[macro_use] extern crate serde;
#[macro_use] extern crate psalmer;

extern crate yaml_rust;

mod sermon;
mod worship;
mod psalms;
mod utils;

use crate::psalms::Psalm;

use worship::initiate_sermon_and_start_preaching;

fn main() {
    initiate_sermon_and_start_preaching();
}
