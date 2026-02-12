use std::path::PathBuf;

use clap::Parser;


#[derive(Parser)]
pub struct Opts {
    pub args: Vec<String>,
    
    
    pub config: Option<PathBuf>,
    pub pwd: Option<PathBuf>,
}