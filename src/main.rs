mod config;
mod preacher;
mod psalms;

use crate::psalms::{yaml::YamlPsalm, Psalm};
use crate::config::parse_args;

fn main() {
    let args = parse_args();

    let psalm: YamlPsalm = YamlPsalm {};
    psalm.invoke(psalms::yaml::YamlContext { });

    println!("arg: {:#?}", args)
}
