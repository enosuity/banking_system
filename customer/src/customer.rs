
// use bank::account::Account;

#[derive(Debug)]
pub struct Customer {
  pub customer_id: u32,
  pub name: String,
  pub accounts: Vec<String>,
}

impl Customer {
  pub fn new(customer_id: u32, name: String) -> Self {
    Self {
      customer_id: customer_id,
      name: name,
      accounts: vec![],
    }
  }

  pub fn add_account(&mut self, account_number: &str) {
    self.accounts.push(account_number.to_string());
  }

  pub fn accounts_list(&self) {
    println!("{} has {} accounts -", self.name, self.accounts.len());

    for account in &self.accounts {
      println!("account ====================> {}", account);
    }
  }
}