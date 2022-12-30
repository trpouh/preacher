pub use clap::Parser;
use uuid::Uuid;

use crate::sermon;
#[derive(Parser)]
#[command(author, version, about, long_about = None)]

//TODO: split up input and output
pub struct Worship {

    #[arg(short, long)]
    pub repo: Option<String>,

    #[arg(short, long)]
    pub branch: Option<String>,

    #[arg(long, default_value_t = String::from("./"))]
    pub source_folder: String,
    
    #[arg(short, long, default_value_t = String::from("sermon.yaml"))]
    pub sermon: String,

    //TODO: sermon_dir is still strange ... 
    #[arg(long, default_value_t = String::from(".preacher/tmp"))]
    pub worship_dir: String,

    #[arg(short, long, default_value_t = String::from("./"))]
    pub target_folder: String,
}

pub fn initiate_sermon_and_start_preaching () {

    let mut worship = Worship::parse();
    
    worship.worship_dir = format!("{}/{}", worship.worship_dir, Uuid::new_v4());
    
    match sermon::initialize(&worship) {
        Ok(sermon) => sermon.preach(&worship),
        Err(err) => error!("Hallelujah! Couldn't start preaching because of: {}", err)
    }

    /*
    if let Ok(_) = std::fs::remove_dir_all(worship.worship_dir) {
        info!("Cleanup finished. The worship is over.");
    } */

}