use std::collections::BTreeMap;

use chrono::{DateTime, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
/// Actions to perform upon the blockchain.
pub mod operations;

#[derive(Clone, Deserialize, Serialize)]
pub struct Block {
    index: u64,
    timestamp: DateTime<Utc>,
    transactions: Vec<Transaction>,
    previous_hash: Vec<u8>,
    proof: u8,
}

#[derive(Clone, Deserialize, Serialize)]
pub enum Transaction {
    /// Grant a user coins when they successfully mine them.
    Grant { to: String, amount: i64 },
    /// Wire money from one user to another.
    Wire {
        from: String,
        to: String,
        amount: i64,
    },
}

pub struct Blockchain {
    length: u64,
    /// The amount to grant a user when they successfully guess the proof number.
    grant_amount: i64,
    blocks: Vec<Block>,
}
impl Blockchain {
    fn genesis_block() -> Block {
        Block {
            index: 0,
            timestamp: Utc::now(),
            transactions: vec![],
            previous_hash: b"genesis_genesis_genesis_genesis_".to_vec(),
            proof: 253,
        }
    }
    /// Closes the current block and starts a new one.
    pub fn commit(&mut self) {
        self.length += 1;
        // Serialize the last block for hashing.
        let last_block_ref = self.blocks.last().unwrap();
        let last_block_bytes = bincode::serialize(&last_block_ref).unwrap();
        // Hash.
        let mut hasher = Sha256::new();
        hasher.update(&last_block_bytes);
        let previous_hash = hasher.finalize().to_vec();
        // Get a new proof number.
        let mut random = rand::thread_rng();
        let proof: u8 = random.gen();
        let new_block = Block {
            index: self.length,
            proof,
            previous_hash,
            timestamp: Utc::now(),
            transactions: vec![],
        };
        self.blocks.push(new_block);
    }

    pub fn new() -> Self {
        let mut chain = Self {
            length: 0,
            grant_amount: 4,
            blocks: vec![],
        };
        chain.blocks.push(Self::genesis_block());
        chain
    }
}
