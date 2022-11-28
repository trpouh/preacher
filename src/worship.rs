pub use clap::Parser;
use uuid::Uuid;
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Worship {

    #[arg(short, long)]
    pub repo: Option<String>,

    #[arg(short, long)]
    pub branch: Option<String>,

    #[arg(long, default_value_t = String::from("./"))]
    pub source_folder: String,
    
    #[arg(short, long, default_value_t = String::from("sermon.yaml"))]
    pub sermon: String,
    
    #[arg(short, long, default_value_t = String::from("./"))]
    pub target_folder: String,

    #[arg(long, default_value_t = String::from("~/.preacher/tmp"))]
    pub tmp_dir: String,
}

pub fn parse_args () -> Worship {

    let mut worship = Worship::parse();
    
    worship.tmp_dir = format!("{}/{}", worship.tmp_dir, Uuid::new_v4());
    
    worship

}
