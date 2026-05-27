use ed25519_dalek::{Signature, Signer, SigningKey};
use rand::rngs::OsRng;

pub struct Wallet {
    signing_key: SigningKey,
}

impl Wallet {
    pub fn new() -> Self {
        Wallet {
            signing_key: SigningKey::generate(&mut OsRng),
        }
    }

    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        let signature: Signature = self.signing_key.sign(message);
        signature.to_bytes().to_vec()
    }

    pub fn public_key(&self) -> Vec<u8> {
        self.signing_key.verifying_key().to_bytes().to_vec()
    }
}