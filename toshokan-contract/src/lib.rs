#![no_std]
use soroban_sdk::{
contract,
contractimpl,
contracttype,
Address,
Env,
String,
};
#[contracttype]
#[derive(Clone)]
pub struct Loan {
pub customer: String,
pub book: String,
pub loan_date: u64,
pub due_date: u64,
}
#[contract]
pub struct ToshokanContract;
#[contractimpl]
impl ToshokanContract {
     pub fn register_loan(
 env: Env,
user: Address,
customer: String,
book: String,
 loan_date: u64,
 due_date: u64,
 ) {
    let loan = Loan {
    customer,
    book,
    loan_date,
    due_date,
    };
    env.storage()
    .persistent()
    .set(&user, &loan);
     }
      pub fn get_loan(
         env: Env,
         user: Address,
          ) -> Loan {
            env.storage()
            .persistent()
            .get(&user)
            .unwrap()
        }
pub fn calculate_fine(
 env: Env,
 user: Address,
) -> i128 {
 let loan: Loan =
 env.storage()
 .persistent()
 .get(&user)
 .unwrap();
 let current_time =
 env.ledger().timestamp();
 if current_time <= loan.due_date {
 return 0;
 }
 let overdue_days =
 (current_time - loan.due_date)
 / 86400;
 (overdue_days as i128) * 2
 }
}