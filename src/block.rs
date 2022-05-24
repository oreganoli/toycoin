use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct Block {
    index: u64,
    timestamp: DateTime<Utc>,
    transactions: Vec<Transaction>,
    previous_hash: [u8; 32],
    proof: u8,
}

#[derive(Deserialize, Serialize)]
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
