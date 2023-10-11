
use crate::transaction::Transaction;

#[derive(Debug)]
pub struct AccountStatement {
  pub account_number: String,
  pub transactions: Vec<Transaction>
}

impl AccountStatement {
  pub fn new(acc: &str) -> Self {
    Self {
      account_number: acc.to_string(),
      transactions: vec![],
    }
  }

  pub fn add_transaction(&mut self, transaction: &Transaction) {
    self.transactions.push(transaction.clone());
  }
}