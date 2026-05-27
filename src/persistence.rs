use std::fs;
use serde_json::{json, Value};
use crate::blockchain::Blockchain;
use crate::transaction::Transaction;

pub fn save_chain(bc: &Blockchain) {
    let blocks: Vec<Value> = bc.chain().iter().map(|b| {
        let txs: Vec<Value> = b.transactions().iter().map(|tx| {
            json!({
                "sender":   tx.sender(),
                "receiver": tx.receiver(),
                "amount":   tx.amount(),
                "currency": tx.currency(),
                "tx_id":    tx.tx_id(),
            })
        }).collect();

        json!({
            "index":         b.index(),
            "timestamp":     b.timestamp(),
            "previous_hash": b.previous_hash(),
            "hash":          b.hash(),
            "transactions":  txs,
        })
    }).collect();

    fs::write("chain.json", serde_json::to_string_pretty(&blocks).unwrap()).unwrap();
}

pub fn load_chain() -> Blockchain {
    let mut bc = Blockchain::load_new();

    let data = match fs::read_to_string("chain.json") {
        Ok(s)  => s,
        Err(_) => return bc,
    };

    let blocks: Vec<Value> = serde_json::from_str(&data).unwrap();

    for block in &blocks {
        let txs: Vec<Transaction> = block["transactions"]
            .as_array().unwrap()
            .iter()
            .map(|tx| Transaction::new(
                tx["sender"].as_str().unwrap().to_string(),
                tx["receiver"].as_str().unwrap().to_string(),
                tx["amount"].as_u64().unwrap(),
                tx["currency"].as_str().unwrap().to_string(),
            ))
            .collect();

        bc.load_block(
            block["index"].as_u64().unwrap() as u32,
            block["timestamp"].as_u64().unwrap(),
            txs,
            block["previous_hash"].as_str().unwrap().to_string(),
            block["hash"].as_str().unwrap().to_string(),
        );
    }

    bc
}
