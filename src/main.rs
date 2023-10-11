use std::{fs::File, io::{Write, prelude::*}};
use std::hash::{Hasher, Hash, self};

use bank::{ Account, Transaction, Bank, UniqueNumbers };
use customer::{ Customer, CustomerTransaction, TType };

use serde_json;

fn main() -> std::io::Result<()> {

//    let mut bank_ac = Account::new("10002345908783");
//    bank_ac.deposit(1500.0);
//    bank_ac.withdraw(200.0);

//    let mut bank_ac2 = Account::new("20002340008783");
//    bank_ac2.deposit(6500.0);
//    bank_ac.account_info();
//    bank_ac2.account_info();
//    println!("============================= before ===============");

//    let mut customer = Customer::new(1001, "Anuj".to_string());
//    customer.add_account(&bank_ac.account_number);
//    customer.add_account(bank_ac2.account_number.as_str());
//    customer.accounts_list();

//    let bank_transaction = BankTransaction::new(2001, &bank_ac.account_number, &bank_ac2.account_number, 320 as f64);
//    println!("Bank Transaction: {:?}", bank_transaction);

//    let transaction_type = TType::Purchase;

//    let customer_transaction = CustomerTransaction::new(
//         bank_transaction.transaction_id as u32,
//         customer.customer_id,
//         &bank_ac.account_number,
//         transaction_type,
//         450.0
//     );

//     println!("Customer Transaction: {:?}", customer_transaction);

   
   
   
    let mut bank = Bank::new();
    let mut customer = Customer::new(1001, "Jay".to_string());
    bank.create_account("12345678");
    bank.create_account("87654321");

    customer.add_account("12345678");
    customer.add_account("87654321");

    bank.deposit("12345678", 8000f64);
    bank.deposit("12345678", 200f64);
    bank.deposit("12345678", 4000f64);
    bank.deposit("87654321", 700f64);

    bank.transfer("87654321", "12345678", 500.0);
    bank.withdraw("87654321", 300.0);
    // println!("bank =============> {:?}", bank);

    let statements = bank.generate_statement("12345678").expect("Account '12345678' does not exist");

    let path = "/data/bank_storage.json";
    let mut file = File::options().append(true).write(true).open(path);
    


    let json = r#"

    {
      "article": "how to work with json in Rust",
      "author": "tdep",
      "paragraph": [
        {
          "name": "untyped"
        },
        {
          "name": "strongly typed"
        },
        {
          "name": "writing json"
        }
      ]
    }
    
    "#;

    

    let mut file: File = match file {
        Ok(f) => f,
        Err(error) => {
            File::create(path).unwrap()
        }
    };

    file.write_all(json.as_bytes())
        .expect("Failed to write to file");

    println!("========= Ending ================ ");

    // for transaction in statements.transactions {
    //     let json_str = serde_json::to_string(&transaction).unwrap();
    //     // let formatted_string = format!(r#"{" {} "}"#,  json_str );
    //     println!("{}", json_str);
    //     file.write_all(json_str.as_bytes())
    //         .expect("Failed to write to file");

    // }
    
    Ok(())

}
