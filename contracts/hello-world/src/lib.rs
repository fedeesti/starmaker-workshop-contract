#![no_std]

mod storage;
mod types;

use soroban_sdk::{contract, contractimpl, vec, Address, Env, String, Vec};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn hello(env: Env, to: String) -> Vec<String> {
        vec![&env, String::from_str(&env, "Hello"), to]
    }

    pub fn add_admin(env: Env, admin: Address) {
        if storage::has_admin(&env) {
            panic!("El admin ya ha sido asignado");
        }
        storage::write_admin(&env, &admin);
    }

    fn check_admin(env: Env) {
        if !storage::has_admin(&env) {
            panic!("El contrato no tiene un admin asignado");
        }

        let admin = storage::read_admin(&env);

        admin.require_auth();
    }

    pub fn get_admin(env: Env) -> Address {
        storage::read_admin(&env)
    }

    pub fn add_client(env: Env, client: Address, balance: i128) {
        Self::check_admin(env.clone());

        let data = types::Client {
            balance,
            status: types::ClientStatus::Enabled,
        };
        storage::write_client(&env, &client, &data);
    }

    pub fn update_client(env: Env, address: Address, status: bool) {
        Self::check_admin(env.clone());

        if !storage::has_client(&env, &address) {
            panic!("Cliente no encontrado");
        }

        let mut client = storage::read_client(&env, &address);

        if status {
            client.status = types::ClientStatus::Enabled
        } else {
            client.status = types::ClientStatus::Disabled
        };

        storage::write_client(&env, &address, &client);
    }

    pub fn remove_client(env: Env, client: Address) {
        Self::check_admin(env.clone());

        if !storage::has_client(&env, &client) {
            panic!("Cliente no encontrado");
        }

        storage::remove_client(&env, &client);
    }
}

mod test;
