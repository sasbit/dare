use std::collections::HashMap;
use crate::block::Block;
use crate::transaction::Transaction;

pub struct Blockchain {
    chain: Vec<Block>,
    balances: HashMap<String, HashMap<String, u64>>,
    public_keys: HashMap<String, Vec<u8>>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis = Block::new(0, vec![], String::from("0"));
        Blockchain {
            chain: vec![genesis],
            balances: HashMap::new(),
            public_keys: HashMap::new(),
        }
    }

    pub fn load_new() -> Self {
        Blockchain {
            chain: vec![],
            balances: HashMap::new(),
            public_keys: HashMap::new(),
        }
    }

    pub fn register_key(&mut self, address: String, public_key: Vec<u8>) {
        self.public_keys.insert(address, public_key);
    }

    pub fn fund_address(&mut self, address: String, currency: String, amount: u64) {
        *self.balances.entry(address).or_default()
            .entry(currency).or_insert(0) += amount;
    }

    pub fn get_balance(&self, address: &str, currency: &str) -> u64 {
        self.balances.get(address)
        .and_then(|c| c.get(currency))
        .copied()
        .unwrap_or(0)
    }

    pub fn chain(&self) -> &[Block] { &self.chain }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) -> Result<(), String> {
        // pass 1: verify signatures
        for tx in &transactions {
            let key = self.public_keys.get(tx.sender())
                .ok_or_else(|| format!("{} has no registered key", tx.sender()))?;
            if !tx.verify(key) {
                return Err(format!("invalid signature from {}", tx.sender()));
            }
        }

        // pass 2: verify balances
        for tx in &transactions {
            if self.get_balance(tx.sender(), tx.currency()) < tx.amount() {
                return Err(format!("{} has insufficient funds", tx.sender()));
            }
        }

        // pass 3: apply balances and push block
        for tx in &transactions {
            *self.balances.entry(tx.sender().to_string()).or_default()
                .entry(tx.currency().to_string()).or_insert(0) -= tx.amount();
            *self.balances.entry(tx.receiver().to_string()).or_default()
                .entry(tx.currency().to_string()).or_insert(0) += tx.amount();
        }

        let index = self.chain.len() as u32;
        let prev_hash = self.chain.last().unwrap().hash().to_string();
        self.chain.push(Block::new(index, transactions, prev_hash));
        Ok(())
    }

    pub fn load_block(&mut self, index: u32, timestamp: u64, transactions: Vec<Transaction>, previous_hash: String, hash: String) {
        self.chain.push(Block::load(index, timestamp, transactions, previous_hash, hash));
        self.rebuild_balances();
    }

    fn rebuild_balances(&mut self) {
        self.balances.clear();
        for block in &self.chain {
            for tx in block.transactions() {
                *self.balances.entry(tx.sender().to_string()).or_default()
                    .entry(tx.currency().to_string()).or_insert(0) -= tx.amount();
                *self.balances.entry(tx.receiver().to_string()).or_default()
                    .entry(tx.currency().to_string()).or_insert(0) += tx.amount();
            }
        }
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            if !self.chain[i].verify_hash() { return false; }
            if !self.chain[i].verify_previous_hash(&self.chain[i - 1]) { return false; }
        }
        true
    }

    pub fn tamper_block(&mut self, index: usize) {
        self.chain[index].corrupt_hash();
    }
}
