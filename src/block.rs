use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};


use crate::merkle::compute_merkle_root;
use crate::transaction::Transaction;


pub struct Block {
    index: u32,
    timestamp: u64,
    transactions: Vec<Transaction>,
    previous_hash: String,
    hash: String,
}

impl Block {
    pub fn new(index: u32, transactions: Vec<Transaction>, previous_hash: String) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let hash = Self::calculate_hash(index, timestamp, &transactions, &previous_hash);

        Block { index, timestamp, transactions, previous_hash, hash }
    }

    pub fn load(index: u32, timestamp: u64, transactions: Vec<Transaction>, previous_hash: String, hash: String) -> Self {
        Block { index, timestamp, transactions, previous_hash, hash }
    }

    pub fn index(&self) -> u32                    { self.index }
    pub fn timestamp(&self) -> u64                { self.timestamp }
    pub fn hash(&self) -> &str                    { &self.hash }
    pub fn previous_hash(&self) -> &str           { &self.previous_hash }
    pub fn transactions(&self) -> &[Transaction]  { &self.transactions }

    pub fn verify_hash(&self) -> bool {
        Self::calculate_hash(self.index, self.timestamp, &self.transactions, &self.previous_hash) == self.hash
    }

    pub fn verify_previous_hash(&self, previous: &Block) -> bool {
        self.previous_hash == previous.hash
    }

    pub fn corrupt_hash(&mut self) {
        self.hash = String::from("corrupted");
    }

    fn calculate_hash(index: u32, timestamp: u64, transactions: &[Transaction], previous_hash: &str) -> String {
        let tx_ids: Vec<String> = transactions.iter().map(|tx| tx.tx_id().to_string()).collect();
        let merkle_root = compute_merkle_root(tx_ids);

        let input = format!("{}{}{}{}", index, timestamp, merkle_root, previous_hash);

        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        hex::encode(hasher.finalize())
    }
}

