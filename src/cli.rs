use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    pub datadir: Option<PathBuf>,
    pub net_addr: Option<String>,
    pub main_addr: Option<String>,
}
