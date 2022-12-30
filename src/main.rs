extern crate serde;
extern crate psalmer;
extern crate yaml_rust;
extern crate env_logger;
extern crate tera;
#[macro_use] extern crate log;


mod sermon;
mod worship;
mod psalms;
mod utils;

use log::info;
use worship::initiate_sermon_and_start_preaching;

fn main() {

    env_logger::init();

    let margin = 1;

    let text = "WELCOME TO THE PREACHER";
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    info!("{}", "-".repeat(text.len()+2*margin));
    info!("{}{}", " ".repeat(margin), text);
    info!("{}v{}", " ".repeat(margin - 1 + ((text.len()-VERSION.len()) / 2)), VERSION);
    info!("{}", "-".repeat(text.len()+2*margin));

    initiate_sermon_and_start_preaching();
}
