use clap::Parser as _;
use ddk::{authorization::Authorization, node::State as _};
use std::net::SocketAddr;
use thunder::{Thunder, ThunderState};

mod cli;
mod thunder;

type Node = ddk::node::Node<Authorization, Thunder, ThunderState>;
type Wallet = ddk::wallet::Wallet<Thunder>;
type Miner = ddk::miner::Miner<Authorization, Thunder>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = cli::Cli::parse();
    const DEFAULT_NET_ADDR: &str = "127.0.0.1:4000";
    let net_addr: SocketAddr = cli
        .net_addr
        .unwrap_or(DEFAULT_NET_ADDR.to_string())
        .parse()?;
    const DEFAULT_MAIN_ADDR: &str = "127.0.0.1:18443";
    let main_addr: SocketAddr = cli
        .main_addr
        .unwrap_or(DEFAULT_MAIN_ADDR.to_string())
        .parse()?;
    let datadir = cli.datadir.unwrap_or_else(|| {
        dirs::data_dir().expect("couldn't get default datadir, specify --datadir")
    });
    let _node = Node::new(&datadir, net_addr, main_addr)?;
    let _wallet = Wallet::new(&datadir.join("wallet.mdb"))?;
    let _miner = Miner::new(ThunderState::THIS_SIDECHAIN, main_addr)?;
    Ok(())
}
