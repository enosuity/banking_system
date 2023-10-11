use crate::{
  Account,
  AccountStatement,
  Transaction,
  UniqueNumbers
};

#[derive(Debug)]
pub struct Bank {
  pub accounts: Vec<Account>,
  pub listing: Vec<u64>,
}

impl Bank {

  pub fn new() -> Self {
    Self {
      accounts: Vec::new(),
      listing: Vec::new()
    }
  }

  pub fn create_account(&mut self, acc: &str) {
    let accnt = Account::new(acc);
    self.accounts.push(accnt);
  }

  pub fn transaction(&mut self, transaction_id: u64, from: &str, to: &str, amount: f64) -> bool {
    for account in &mut self.accounts {
      if account.account_number == from.to_string() || account.account_number == to.to_string() {     
        println!(" ===========  Bank transaction ==================");   
        account.transaction(transaction_id, from, to, amount);        
        return  true;
      }
    }
    true
  }

  pub fn deposit(&mut self, to: &str, amount: f64) -> bool {
    for account in &mut self.accounts {
      if account.account_number == to.to_string() {
        
        account.deposit(amount);
        
        let transaction_id = self.unik_transaction_id();
        println!("transaction_id ==========> {:?}", transaction_id);

        if self.transaction(transaction_id, "", to, amount) {
          println!("============ Yes ================== ");
          self.listing.push(transaction_id);
          return  true;
        }
        return false;
      }
    }
    false
  }

  pub fn withdraw(&mut self, from: &str, amount: f64) -> bool {
    for account in &mut self.accounts {
      if account.account_number == from.to_string() {
        account.withdraw(amount);

        let transaction_id = self.unik_transaction_id();
        println!("transaction_id ==========> {:?}", transaction_id);

        if self.transaction(transaction_id, from, "", amount) {
          println!("============ Withdraw Yes ================== ");
          self.listing.push(transaction_id);
          return  true;
        }


        return true;
      }
    }
    false
  }

  pub fn transfer(&mut self, to: &str, from: &str, amount: f64) -> bool{
    if self.withdraw(from, amount) {
      return self.deposit(to, amount);
    } 
    false 
  }

  pub fn generate_statement(&mut self, acc: &str) -> Option<AccountStatement> {
    for account in  &self.accounts {
      
      if account.account_number == acc.to_string() {
        let mut statement = AccountStatement::new(acc);

        for transaction in &account.transactions {
          statement.add_transaction(&transaction.clone());
        }
        return Some(statement);
      }
    }         
    None
  }

  fn unik_transaction_id(&mut self) -> u64 {
    let trans_id = UniqueNumbers::generate();
    if self.listing.contains(&trans_id) {
      println!("trans_id: {} already existing. re-trying again ..", trans_id);
      return self.unik_transaction_id();
    }
    trans_id
  }
}