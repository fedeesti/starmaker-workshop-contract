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
