use std::collections::BTreeMap;

use super::{Block, Blockchain, Transaction};

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
}
