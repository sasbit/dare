use serde::{Serialize, Deserialize};
use crate::block::Block;
use crate::transaction::Transaction;

#[derive(Serialize, Deserialize, Clone)]
pub enum Message {
    NewBlock(Block),
    NewTransaction(Transaction),
    RequestChain,
    ResponseChain(Vec<Block>),
    Hello { from: String },
}
