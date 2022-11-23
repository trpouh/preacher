pub use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Invocation {

    pub repo: Option<String>,
    
    #[arg(short, long, default_value_t = String::from("sermon.yaml"))]
    pub sermon: String,
    
    #[arg(short, long, default_value_t = String::from("./"))]
    pub run_in_dir: String
}

pub fn parse_args () -> Invocation {
    Invocation::parse()
}
