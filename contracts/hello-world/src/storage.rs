use soroban_sdk::{Address, Env};

use crate::types::{Client, DataKey};

pub fn has_admin(env: &Env) -> bool {
    env.storage().persistent().has(&DataKey::Admin)
}

pub fn read_admin(env: &Env) -> Address {
    env.storage().persistent().get(&DataKey::Admin).unwrap()
}

pub fn write_admin(env: &Env, admin: &Address) {
    env.storage().persistent().set(&DataKey::Admin, admin);
}

pub fn has_client(env: &Env, client: &Address) -> bool {
    env.storage()
        .persistent()
        .has(&DataKey::Client(client.clone()))
}

pub fn read_client(env: &Env, client: &Address) -> Client {
    env.storage()
        .persistent()
        .get(&DataKey::Client(client.clone()))
        .unwrap()
}

pub fn write_client(env: &Env, client: &Address, data: &Client) {
    env.storage()
        .persistent()
        .set(&DataKey::Client(client.clone()), data);
}

pub fn remove_client(env: &Env, client: &Address) {
    env.storage()
        .persistent()
        .remove(&DataKey::Client(client.clone()));
}

pub fn has_recieve(env: &Env, recieve: &Address) -> bool {
    env.storage()
        .persistent()
        .has(&DataKey::Recieve(recieve.clone()))
}

pub fn read_recieve(env: &Env, recieve: &Address) -> i128 {
    env.storage()
        .persistent()
        .get(&DataKey::Recieve(recieve.clone()))
        .unwrap()
}

pub fn write_recieve(env: &Env, recieve: &Address, balance: &i128) {
    env.storage()
        .persistent()
        .set(&DataKey::Recieve(recieve.clone()), &balance);
}

pub fn remove_recieve(env: &Env, recieve: &Address) {
    env.storage()
        .persistent()
        .remove(&DataKey::Recieve(recieve.clone()));
}

pub fn read_token(env: &Env) -> Address {
    env.storage().persistent().get(&DataKey::Token).unwrap()
}

pub fn write_token(env: &Env, token_address: &Address) {
    env.storage()
        .persistent()
        .set(&DataKey::Token, &token_address);
}

pub fn read_contract_balance(env: &Env) -> i128 {
    env.storage()
        .persistent()
        .get(&DataKey::ContractBalance)
        .unwrap_or(0)
}

pub fn write_contract_balance(env: &Env, amount: &i128) {
    env.storage()
        .persistent()
        .set(&DataKey::ContractBalance, amount);
}
