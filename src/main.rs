#[macro_use] extern crate serde;
#[macro_use] extern crate psalmer;

extern crate yaml_rust;

mod sermon;
mod worship;
mod psalms;
mod utils;

use crate::sermon::initialize;
use crate::worship::parse_args;
use crate::psalms::Psalm;

use worship::Worship;

fn main() {

    let worship: Worship = parse_args();

    let _sermon = initialize(&worship);

    match _sermon {
        Ok(sermon) => sermon.preach(&worship),
        Err(err) => println!("Hallelujah! Couldn't start preaching because of: {}", err)
    }
}
