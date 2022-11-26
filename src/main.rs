extern crate yaml_rust;

mod sermon;
mod worship;
mod psalms;

use crate::sermon::initialize;
use crate::worship::parse_args;
use crate::psalms::Psalm;

use worship::Worship;

fn main() {

    let worship: Worship = parse_args();

    let _sermon = initialize(&worship);

    if let Ok(sermon) = _sermon {
        sermon.preach(&worship);
    }
}
