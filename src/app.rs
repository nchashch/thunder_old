use crate::cli::Config;
use crate::thunder;
use ddk::node::State as _;
use thunder::{Miner, Node, ThunderState, Wallet};

pub struct App {
    node: Node,
    wallet: Wallet,
    miner: Miner,
}

impl App {
    pub fn new(config: &Config) -> Result<Self, Error> {
        // Node launches some tokio tasks for p2p networking, that is why we need a tokio runtime
        // here.
        let node = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()?
            .block_on(async { Node::new(&config.datadir, config.net_addr, config.main_addr) })?;
        let wallet = Wallet::new(&config.datadir.join("wallet.mdb"))?;
        let miner = Miner::new(ThunderState::THIS_SIDECHAIN, config.main_addr)?;
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
    #[error("io error")]
    Io(#[from] std::io::Error),
}
