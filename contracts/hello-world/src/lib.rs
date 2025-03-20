#![no_std]

mod storage;
mod token;
mod types;

use soroban_sdk::{contract, contractimpl, Address, Env};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn initialize(env: Env, admin: Address, token: Address) {
        if storage::has_admin(&env) {
            panic!("El contrato ya ha sido inicializado");
        }

        storage::write_admin(&env, &admin);
        storage::write_token(&env, &token);
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

    pub fn add_recieve(env: Env, recieve: Address, balance: i128) {
        Self::check_admin(env.clone());

        storage::write_recieve(&env, &recieve, &balance);
    }

    pub fn remove_recieve(env: Env, recieve: Address) {
        Self::check_admin(env.clone());

        if !storage::has_recieve(&env, &recieve) {
            panic!("Recieve no encontrado");
        }

        storage::remove_recieve(&env, &recieve);
    }

    pub fn amount_to_withdraw(env: Env, recieve: Address) -> i128 {
        if !storage::has_recieve(&env, &recieve) {
            panic!("Recieve no encontrado");
        }

        storage::read_recieve(&env, &recieve)
    }

    pub fn deposit(env: Env, from: Address, to: Address, amount: i128) {
        if !storage::has_client(&env, &from) {
            panic!("Cliente no encontrado");
        }
        let mut from_client = storage::read_client(&env, &from);

        if !storage::has_recieve(&env, &to) {
            panic!("Recieve no encontrado");
        }
        let mut contract_balance = storage::read_contract_balance(&env);

        token::token_transfer(&env, &from, &env.current_contract_address(), &amount);

        from_client.balance -= amount;
        storage::write_client(&env, &from, &from_client);

        contract_balance += amount;
        storage::write_contract_balance(&env, &contract_balance);
    }

    pub fn withdraw(env: Env, recieve: Address, amount: i128) {
        if !storage::has_recieve(&env, &recieve) {
            panic!("Recieve no encontrado");
        }

        let mut to_balance = storage::read_recieve(&env, &recieve);

        if to_balance < amount {
            panic!("Fondos insuficientes");
        }

        let mut contract_balance = storage::read_contract_balance(&env);

        if contract_balance < amount {
            panic!("Fondos insuficientes del contrato");
        }

        token::token_transfer(&env, &env.current_contract_address(), &recieve, &amount);

        to_balance -= amount;
        contract_balance -= amount;

        storage::write_recieve(&env, &recieve, &to_balance);
        storage::write_contract_balance(&env, &contract_balance);
    }
}

mod test;
