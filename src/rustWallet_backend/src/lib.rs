use ic_cdk::api::caller;
use ic_cdk::export::Principal;
use ic_cdk::storage::{stable, Stable};
use ic_cdk_macros::*;

#[derive(Default, Stable)]
pub struct Token {
    pub balances: std::collections::HashMap<Principal, u64>,
}

#[init]
fn init() {
    let mut token = Token::default();
    token.balances.insert(caller(), 1_000_000); // Initial supply
    stable_save((token,)).unwrap();
}

#[update]
fn transfer(to: Principal, amount: u64) -> Result<(), String> {
    let (token,) = stable_restore().unwrap();
    let from = caller();

    if let Some(balance) = token.balances.get_mut(&from) {
        if *balance < amount {
            return Err("Insufficient funds".to_string());
        }
        *balance -= amount;
        *token.balances.entry(to).or_insert(0) += amount;
        stable_save((token,)).unwrap();
        Ok(())
    } else {
        Err("Sender does not have an account".to_string())
    }
}

#[query]
fn balance_of(account: Principal) -> u64 {
    let (token,) = stable_restore().unwrap();
    *token.balances.get(&account).unwrap_or(&0)
}

#[query]
fn total_supply() -> u64 {
    let (token,) = stable_restore().unwrap();
    token.balances.values().sum()
}