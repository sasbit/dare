use sha2::{Digest, Sha256};
use serde::{Serialize, Deserialize};
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use crate::wallet::Wallet;
use std::convert::TryInto;

#[derive(Serialize, Deserialize,Clone)]



pub struct Transaction {
    sender: String,
    receiver: String,
    amount: u64,
    currency: String,
    tx_id: String,
    signature: Option<Vec<u8>>,
}

impl Transaction {
    pub fn new(sender: String, receiver: String, amount: u64, currency: String) -> Self {
        let tx_id = Self::compute_id(&sender, &receiver, amount, &currency);
        Transaction { sender, receiver, amount, currency, tx_id, signature: None }
        
    }

    fn compute_id (sender: &str, receiver: &str, amount: u64, currency: &str) -> String {
        let input = format!("{}{}{}{}", sender, receiver, amount, currency);
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        hex::encode(hasher.finalize())
    }

    //public getter methods
    pub fn sender(&self) -> &str { &self.sender }
    pub fn receiver(&self) -> &str { &self.receiver}
    pub fn amount(&self) -> u64 { self.amount }
    pub fn currency(&self) -> &str { &self.currency }
    pub fn tx_id(&self) -> &str { &self.tx_id }

    pub fn sign(&mut self, wallet: &Wallet) {
        let message = format!("{}{}{}{}", self.sender, self.receiver, self.amount, self.currency);
        self.signature = Some(wallet.sign(message.as_bytes()));
    }

    pub fn verify(&self, public_key: &[u8]) -> bool {
        let sig_bytes = match &self.signature {
            Some(s) => s,
            None => return false,
        };
        
        let Ok(key_array) = TryInto::<&[u8; 32]>::try_into(public_key.as_ref()) else { return false };
        let Ok(sig_array) = TryInto::<&[u8; 64]>::try_into(sig_bytes.as_slice()) else { return false };

        let Ok(verifying_key) = VerifyingKey::from_bytes(key_array) else { return false };
        let signature = Signature::from_bytes(sig_array);

        let message = format!("{}{}{}{}", self.sender, self.receiver, self.amount, self.currency);
        verifying_key.verify(message.as_bytes(), &signature).is_ok()
    }
}