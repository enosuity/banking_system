
#[derive(Debug)]
pub struct CustomerTransaction {
  pub transaction_id:   u32,
  pub customer_id:      u32,
  pub account_number:   String,
  pub transaction_type: TType,
  pub amount:           f64,
  pub description:      String,
}

#[derive(Debug)]
pub enum TType {
  Purchase,
  Transfer
}

impl CustomerTransaction {
  pub fn new(transaction_id: u32, customer_id: u32, account_number: &str, transaction_type: TType, amount: f64, description: &str) -> Self {
    Self {
      transaction_id,
      customer_id,
      account_number: account_number.to_string(),
      transaction_type,
      amount,
      description: description.to_string(),
    }
  }
}