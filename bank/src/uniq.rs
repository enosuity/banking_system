use rand::prelude::*;

#[derive(Debug)]
pub struct UniqueNumbers;

impl UniqueNumbers {  
  pub fn generate() -> u64 {
    let mut rng = thread_rng();
    let tran_id: u64 = rng.gen_range(1..=1000000);
    // if list.contains(&tran_id) {
    //   println!("trans_id: {} already existing. re-trying again ..", tran_id);
    //   return Self::generate(list);      
    // }
    tran_id
  }
}