use serde::Serialize;


#[derive(Debug, Clone, Serialize)]
pub struct Transaction {
  pub transaction_id: u64,
  pub from:           String,
  pub to:             String,
  pub amount:         f64,
}

impl Transaction {
  pub fn new(transaction_id: u64, from: &str, to: &str, amount: f64) -> Self{
    Self {
      transaction_id,
      from: from.to_string(),
      to: to.to_string(),
      amount,
    }
  }
}