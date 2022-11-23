mod bible;
mod config;
mod psalms;

use crate::bible::{initialize, Bible};
use crate::config::parse_args;
use crate::psalms::Psalm;

use config::Invocation;

fn main() {
    let invocation: Invocation = parse_args();

    let _bible = initialize(invocation);

    // println!("Bible: {:#?}", _bible);

    if let Ok(bible) = _bible {
        bible.preach();
    }
}
