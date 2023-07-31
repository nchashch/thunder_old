use ddk::heed;
use ddk::node::State;
use ddk::types::GetValue;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Thunder;

impl GetValue for Thunder {
    fn get_value(&self) -> u64 {
        0
    }
}

#[derive(Clone)]
pub struct ThunderState;

impl ThunderState {
    fn transaction_size_limit(_height: u32) -> u64 {
        1024 * 1024
    }

    fn transaction_sigops_limit(_height: u32) -> u64 {
        8 * 1024
    }

    fn body_size_limit(_height: u32) -> u64 {
        8 * 1024 * 1024
    }

    fn body_sigops_limit(_height: u32) -> u64 {
        8 * 8 * 1024
    }
}

impl State<ddk::authorization::Authorization, Thunder> for ThunderState {
    const THIS_SIDECHAIN: u8 = 0;
    const NUM_DBS: u32 = 5;

    type Error = Error;

    fn new(_env: &heed::Env) -> Result<Self, Self::Error> {
        Ok(Self)
    }

    fn validate_filled_transaction(
        &self,
        _txn: &heed::RoTxn,
        height: u32,
        _state: &ddk::state::State<ddk::authorization::Authorization, Thunder>,
        transaction: &ddk::types::FilledTransaction<Thunder>,
    ) -> Result<(), Self::Error> {
        if transaction.transaction.inputs.len() as u64
            > ThunderState::transaction_sigops_limit(height)
        {
            return Err(Error::TooManySigOpsInTransaction);
        }
        let serialized_transaction = bincode::serialize(&transaction.transaction)?;
        if serialized_transaction.len() as u64 > ThunderState::transaction_size_limit(height) {
            return Err(Error::TransactionTooBig);
        }
        Ok(())
    }

    fn validate_body(
        &self,
        _txn: &heed::RoTxn,
        height: u32,
        _state: &ddk::state::State<ddk::authorization::Authorization, Thunder>,
        body: &ddk::types::Body<ddk::authorization::Authorization, Thunder>,
    ) -> Result<(), Self::Error> {
        if body.authorizations.len() as u64 > ThunderState::body_sigops_limit(height) {
            return Err(Error::TooManySigOpsInBody);
        }
        let serialized_body = bincode::serialize(body)?;
        if serialized_body.len() as u64 > ThunderState::body_size_limit(height) {
            return Err(Error::BodyTooBig);
        }
        Ok(())
    }

    fn connect_body(
        &self,
        _txn: &mut heed::RwTxn,
        _height: u32,
        _state: &ddk::state::State<ddk::authorization::Authorization, Thunder>,
        _body: &ddk::types::Body<ddk::authorization::Authorization, Thunder>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("bincode error")]
    Bincode(#[from] bincode::Error),
    #[error("too many sigops in body")]
    TooManySigOpsInBody,
    #[error("too many sigops in transaction")]
    TooManySigOpsInTransaction,
    #[error("body too big")]
    BodyTooBig,
    #[error("transaction too big")]
    TransactionTooBig,
}

impl ddk::node::CustomError for Error {}

pub type Node = ddk::node::Node<ddk::authorization::Authorization, Thunder, ThunderState>;
pub type Wallet = ddk::wallet::Wallet<Thunder>;
pub type Miner = ddk::miner::Miner<ddk::authorization::Authorization, Thunder>;
