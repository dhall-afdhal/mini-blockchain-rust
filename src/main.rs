use chrono::prelude::*;
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Block {
    index: u64,
    timestamp: String,
    data: String,
    previous_hash: String,
    hash: String,
}

impl Block {
    fn new(index: u64, data: String, previous_hash: String) -> Self {
        let timestamp = Utc::now().to_rfc3339();
        let hash = Block::calculate_hash(index, &timestamp, &data, &previous_hash);
        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
        }
    }

    fn calculate_hash(index: u64, timestamp: &str, data: &str, previous_hash: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(index.to_string());
        hasher.update(timestamp);
        hasher.update(data);
        hasher.update(previous_hash);
        format!("{:x}", hasher.finalize())
    }
}

#[derive(Debug)]
struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    fn new() -> Self {
        let genesis_block = Block::new(0, "Genesis Block".to_string(), "0".to_string());
        Blockchain {
            chain: vec![genesis_block],
        }
    }

    fn add_block(&mut self, data: String) {
        let previous_block = self.chain.last().unwrap().clone();
        let new_block = Block::new(
            previous_block.index + 1,
            data,
            previous_block.hash.clone(),
        );
        self.chain.push(new_block);
    }

    fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            if current.hash
                != Block::calculate_hash(
                    current.index,
                    &current.timestamp,
                    &current.data,
                    &current.previous_hash,
                )
            {
                return false;
            }

            if current.previous_hash != previous.hash {
                return false;
            }
        }
        true
    }
}

fn main() {
    let mut blockchain = Blockchain::new();

    println!("🚀 Mini Blockchain in Rust");
    println!("============================");

    blockchain.add_block("Transaksi: Alice -> Bob (10 Coin)".to_string());
    blockchain.add_block("Transaksi: Bob -> Charlie (5 Coin)".to_string());
    blockchain.add_block("Transaksi: Charlie -> Dave (3 Coin)".to_string());

    for block in &blockchain.chain {
        println!(
            "\n🧱 Block #{} \nTimestamp: {}\nData: {}\nPrev Hash: {}\nHash: {}",
            block.index, block.timestamp, block.data, block.previous_hash, block.hash
        );
    }

    println!(
        "\n🔒 Blockchain valid? {}\n",
        if blockchain.is_valid() { "✅ Ya" } else { "❌ Tidak" }
    );
}
