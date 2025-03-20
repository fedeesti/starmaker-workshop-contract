use crate::storage;

use soroban_sdk::{
    token::{self},
    Address, Env,
};

pub fn token_transfer(env: &Env, from: &Address, to: &Address, amount: &i128) {
    let token_address = storage::read_token(env);
    let token = token::TokenClient::new(env, &token_address);
    token.transfer(from, to, amount);
}
