use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct WireRequest {
    pub from: String,
    pub to: String,
    pub amount: i64,
}

#[derive(Serialize, Deserialize)]
pub struct GuessRequest {
    pub miner: String,
    pub guess: u8,
}
