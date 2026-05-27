use sha2::{Digest, Sha256};

pub fn compute_merkle_root(mut hashes: Vec<String>) -> String {
    if hashes.is_empty() {
        return String::from("0");
    }

    while hashes.len() > 1 {
        if hashes.len() % 2 != 0 {
            let last = hashes.last().unwrap().clone();
            hashes.push(last);
        }

        hashes = hashes
            .chunks(2)
            .map(|pair| hash_pair(&pair[0], &pair[1]))
            .collect();
    }

    hashes.remove(0)
}

fn hash_pair(a: &str, b: &str) -> String {
    let input = format!("{}{}", a, b);
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    hex::encode(hasher.finalize())
}
