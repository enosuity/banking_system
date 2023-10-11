use crate::{Transaction};

#[derive(Debug, Clone)]
pub struct Account {
  pub account_number: String,
  pub balance: f64,
  pub transactions: Vec<Transaction>,
}

impl Account {
  pub fn new(ac: &str) -> Self {

    Self {
      account_number: ac.to_string(),
      balance: 0.0 as f64,
      transactions: vec![],
    }
  }

  

  pub fn deposit(&mut self, amount: f64) {
    self.balance += amount;
  }

  pub fn withdraw(&mut self, amount: f64) -> bool {
    if self.balance >= amount {
      self.balance -= amount;
      return true;
    } 
    false
  }

  pub fn transaction(&mut self, transaction_id: u64, from: &str, to: &str, amount: f64) -> bool {
    println!(" =========== Yes  Account transaction ==================from : {:?} & to: {:?}", from, to);
    
    if self.account_number != from.to_string() && self.account_number != to.to_string() {
      println!(
        "Sorry, Transaction can not be registered because of account number : {:?} and from : {:?} both are different.",
        self.account_number,
        from
      );
      return false;   
    }  
    else {
      println!(" =========== Yes  Account transaction ==================from : {:?} & to: {:?}", from, to);
      self.transactions.push(
        Transaction::new(transaction_id, from, to, amount)
      );
      return true;
    }
  }

  pub fn account_info(&self) {
    println!("{} current balance => {} USDT", self.account_number, self.balance)
  }
}