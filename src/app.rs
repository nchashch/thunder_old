use crate::cli::Config;
use crate::thunder;
use ddk::bitcoin;
use ddk::drivechain::MainClient;
use ddk::jsonrpsee;
use ddk::node::State as _;
use thunder::{Miner, Node, ThunderState, Wallet};

pub struct App {
    pub node: Node,
    pub wallet: Wallet,
    pub miner: Miner,
    runtime: tokio::runtime::Runtime,
}

impl App {
    pub fn new(config: &Config) -> Result<Self, Error> {
        // Node launches some tokio tasks for p2p networking, that is why we need a tokio runtime
        // here.
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()?;
        let wallet = Wallet::new(&config.datadir.join("wallet.mdb"))?;
        let miner = Miner::new(ThunderState::THIS_SIDECHAIN, config.main_addr)?;
        let node = runtime.block_on(async {
            let node = match Node::new(&config.datadir, config.net_addr, config.main_addr) {
                Ok(node) => node,
                Err(err) => return Err(err),
            };
            Ok(node)
        })?;
        Ok(Self {
            node,
            wallet,
            miner,
            runtime,
        })
    }

    const EMPTY_BLOCK_BMM_BRIBE: u64 = 1000;
    pub fn mine(&mut self) -> Result<(), Error> {
        self.runtime.block_on(async {
            const NUM_TRANSACTIONS: usize = 1000;
            let (transactions, fee) = self.node.get_transactions(NUM_TRANSACTIONS)?;
            let coinbase = match fee {
                0 => vec![],
                _ => vec![ddk::types::Output {
                    address: self.wallet.get_new_address()?,
                    content: ddk::types::Content::Value(fee),
                }],
            };
            let body = ddk::types::Body::new(transactions, coinbase);
            let prev_side_hash = self.node.get_best_hash()?;
            let prev_main_hash = self.miner.drivechain.get_mainchain_tip().await?;
            let header = ddk::types::Header {
                merkle_root: body.compute_merkle_root(),
                prev_side_hash,
                prev_main_hash,
            };
            let bribe = if fee > 0 {
                fee
            } else {
                Self::EMPTY_BLOCK_BMM_BRIBE
            };
            let bribe = bitcoin::Amount::from_sat(bribe);
            self.miner
                .attempt_bmm(bribe.to_sat(), 0, header, body)
                .await?;
            self.miner.generate().await?;
            if let Ok(Some((header, body))) = self.miner.confirm_bmm().await {
                self.node.submit_block(&header, &body).await?;
            }
            Ok(())
        })
    }

    pub fn deposit(&mut self, amount: bitcoin::Amount, fee: bitcoin::Amount) -> Result<(), Error> {
        self.runtime.block_on(async {
            let address = self.wallet.get_new_address().unwrap();
            let address = ddk::format_deposit_address(&format!("{address}"));
            self.miner
                .drivechain
                .client
                .createsidechaindeposit(
                    ThunderState::THIS_SIDECHAIN,
                    &address,
                    amount.into(),
                    fee.into(),
                )
                .await?;
            Ok(())
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
    #[error("drivechain error")]
    Drivechain(#[from] ddk::drivechain::Error),
    #[error("io error")]
    Io(#[from] std::io::Error),
    #[error("jsonrpsee error")]
    Jsonrpsee(#[from] jsonrpsee::core::Error),
}
