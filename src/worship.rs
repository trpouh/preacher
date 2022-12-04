pub use clap::Parser;
use uuid::Uuid;

use crate::sermon;
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

    #[arg(long, default_value_t = String::from(".preacher/tmp"))]
    pub tmp_dir: String,
}

pub fn initiate_sermon_and_start_preaching () {

    let mut worship = Worship::parse();
    
    worship.tmp_dir = format!("{}/{}", worship.tmp_dir, Uuid::new_v4());
    
    match sermon::initialize(&worship) {
        Ok(sermon) => sermon.preach(&worship),
        Err(err) => println!("Hallelujah! Couldn't start preaching because of: {}", err)
    }

    if let Ok(_) = std::fs::remove_dir_all(worship.tmp_dir) {
        println!("Cleanup finished. The worship is over.");
    }

}