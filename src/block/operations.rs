use std::collections::BTreeMap;

use super::{Block, Blockchain, BlockchainError, Transaction};

impl Blockchain {
    /// Calculate everyone's balance.
    pub fn balance(&self) -> BTreeMap<String, i64> {
        let mut balance_map: BTreeMap<String, i64> = BTreeMap::new();
        let transactions = self.blocks.iter().flat_map(|each| each.transactions.iter());
        for transaction in transactions {
            match transaction {
                Transaction::Grant { to, amount } => match balance_map.get_mut(to) {
                    Some(account) => *account += *amount,
                    None => {
                        balance_map.insert(to.clone(), *amount);
                    }
                },
                Transaction::Wire { from, to, amount } => {
                    match balance_map.get_mut(from) {
                        Some(account) => *account -= *amount,
                        None => {
                            balance_map.insert(from.clone(), -(*amount));
                        }
                    };
                    match balance_map.get_mut(to) {
                        Some(account) => *account += *amount,
                        None => {
                            balance_map.insert(to.clone(), *amount);
                        }
                    };
                }
            }
        }
        balance_map
    }
    /// Return the currently pending transactions.
    pub fn pending(&self) -> Vec<Transaction> {
        self.last_block().transactions.clone()
    }
    /// Guess the proof. Reward the miner if the guess was correct.
    pub fn guess(&mut self, guess: u8, miner: String) -> bool {
        let grant = Transaction::Grant {
            to: miner,
            amount: self.grant_amount,
        };
        let block = self.last_block_mut();
        if guess == block.proof {
            block.transactions.push(grant);
            true
        } else {
            false
        }
    }
    /// Return all the committed blocks, the whole chain up to now.
    pub fn chain(&self) -> Vec<Block> {
        self.blocks
            .iter()
            .take(self.length as usize)
            .map(|block| block.clone())
            .collect::<Vec<_>>()
    }
}
