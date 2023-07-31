use std::{net::SocketAddr, path::Path};

use crate::thunder;
use ddk::node::State as _;
use thunder::{Miner, Node, ThunderState, Wallet};

pub struct App {
    node: Node,
    wallet: Wallet,
    miner: Miner,
}

impl App {
    pub fn new(datadir: &Path, net_addr: SocketAddr, main_addr: SocketAddr) -> Result<Self, Error> {
        let node = Node::new(&datadir, net_addr, main_addr)?;
        let wallet = Wallet::new(&datadir.join("wallet.mdb"))?;
        let miner = Miner::new(ThunderState::THIS_SIDECHAIN, main_addr)?;
        Ok(Self {
            node,
            wallet,
            miner,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("node error")]
    Node(#[from] ddk::node::Error<thunder::Error>),
    #[error("wallet error")]
    Wallet(#[from] ddk::wallet::Error),
    #[error("miner error")]
    Miner(#[from] ddk::miner::Error),
}
