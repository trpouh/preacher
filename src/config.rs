pub use clap::Parser;
use std::env;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Invocation {

    repo: Option<String>,
    
    folder: Option<String>,
    
    #[arg(short, long)]
    psalm: String,
    
    runfromdir: Option<String>,
}

pub fn parse_args () -> Invocation {
    Invocation::parse()
}
