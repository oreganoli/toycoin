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
    /// Validate state.
    pub fn validate_state(&self) -> Result<(), BlockchainError> {
        let balances = self.balance();
        let mut negative_balances: Vec<(String, i64)> = vec![];
        for (account, balance) in balances.iter() {
            if (*balance < 0) {
                negative_balances.push((account.clone(), *balance));
            }
        }
        if negative_balances.is_empty() {
            Ok(())
        } else {
            Err(BlockchainError::NegativeBalances(negative_balances))
        }
    }
}
