use crate::cli::Config;
use crate::thunder;
use ddk::bitcoin;
use ddk::node::State as _;
use thunder::{Miner, Node, ThunderState, Wallet};

pub struct App {
    pub node: Node,
    pub wallet: Wallet,
    pub miner: Miner,
    pub mine_tx: tokio::sync::mpsc::Sender<()>,
    _runtime: tokio::runtime::Runtime,
}

impl App {
    const EMPTY_BLOCK_BMM_BRIBE: u64 = 1000;
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
        let node0 = node.clone();
        let wallet0 = wallet.clone();
        let mut miner0 = miner.clone();
        let (mine_tx, mut mine_rx) = tokio::sync::mpsc::channel(32);
        runtime.spawn(async move {
            loop {
                let _ = mine_rx.recv().await;
                const NUM_TRANSACTIONS: usize = 1000;
                let (transactions, fee) = match node0.get_transactions(NUM_TRANSACTIONS) {
                    Ok(tf) => tf,
                    Err(_) => continue,
                };
                let coinbase = {
                    let address = match wallet0.get_new_address() {
                        Ok(a) => a,
                        Err(_) => continue,
                    };
                    match fee {
                        0 => vec![],
                        _ => vec![ddk::types::Output {
                            address,
                            content: ddk::types::Content::Value(fee),
                        }],
                    }
                };
                let body = ddk::types::Body::new(transactions, coinbase);
                let prev_side_hash = match node0.get_best_hash() {
                    Ok(psh) => psh,
                    Err(_) => continue,
                };
                let prev_main_hash = match miner0.drivechain.get_mainchain_tip().await {
                    Ok(pmh) => pmh,
                    Err(_) => continue,
                };
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
                if miner0
                    .attempt_bmm(bribe.to_sat(), 0, header, body)
                    .await
                    .is_err()
                {
                    continue;
                }
                if miner0.generate().await.is_err() {
                    continue;
                }
                if let Ok(Some((header, body))) = miner0.confirm_bmm().await {
                    if node0.submit_block(&header, &body).await.is_err() {
                        continue;
                    }
                }
            }
        });
        Ok(Self {
            node,
            wallet,
            miner,
            mine_tx,
            _runtime: runtime,
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
