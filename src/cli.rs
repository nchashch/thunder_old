use clap::Parser;
use std::{net::SocketAddr, path::PathBuf};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    pub datadir: Option<PathBuf>,
    pub net_addr: Option<String>,
    pub main_addr: Option<String>,
    pub main_user: Option<String>,
    pub main_password: Option<String>,
}

pub struct Config {
    pub datadir: PathBuf,
    pub net_addr: SocketAddr,
    pub main_addr: SocketAddr,
    pub main_user: String,
    pub main_password: String,
}

impl Cli {
    pub fn get_config(&self) -> anyhow::Result<Config> {
        const DEFAULT_NET_ADDR: &str = "127.0.0.1:4000";
        let net_addr: SocketAddr = self
            .net_addr
            .clone()
            .unwrap_or(DEFAULT_NET_ADDR.to_string())
            .parse()?;
        const DEFAULT_MAIN_ADDR: &str = "127.0.0.1:18443";
        let main_addr: SocketAddr = self
            .main_addr
            .clone()
            .unwrap_or(DEFAULT_MAIN_ADDR.to_string())
            .parse()?;
        let datadir = self
            .datadir
            .clone()
            .unwrap_or_else(|| {
                dirs::data_dir().expect("couldn't get default datadir, specify --datadir")
            })
            .join("thunder");
        let main_user = self.main_user.clone().unwrap_or_else(|| "user".into());
        let main_password = self
            .main_password
            .clone()
            .unwrap_or_else(|| "password".into());
        Ok(Config {
            datadir,
            net_addr,
            main_addr,
            main_user,
            main_password,
        })
    }
}
