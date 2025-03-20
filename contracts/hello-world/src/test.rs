#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Env};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let token = Address::generate(&env);
    client.initialize(&admin, &token);
    assert_eq!(client.get_admin(), admin);
}

#[test]
fn test_add_client() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let new_client = Address::generate(&env);
    client.mock_all_auths().add_admin(&Address::generate(&env));

    client.mock_all_auths().add_client(&new_client, &0);
}

#[test]
#[should_panic(expected = "El contrato no tiene un admin asignado")]
fn test_add_client_no_admin() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let new_client = Address::generate(&env);
    client.add_client(&new_client, &0);
}

#[test]
fn test_update_client() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let new_client = Address::generate(&env);
    client.add_admin(&Address::generate(&env));
    client.mock_all_auths().add_client(&new_client, &0);

    client.mock_all_auths().update_client(&new_client, &true);
}

#[test]
#[should_panic(expected = "El contrato no tiene un admin asignado")]
fn test_update_client_no_admin() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let new_client = Address::generate(&env);
    client.mock_all_auths().add_client(&new_client, &0);

    client.update_client(&new_client, &true);
}

#[test]
#[should_panic(expected = "Cliente no encontrado")]
fn test_update_client_not_found() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.add_admin(&admin);

    let new_client = Address::generate(&env);

    client.mock_all_auths().update_client(&new_client, &true);
}

#[test]
fn test_remove_client() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let new_client = Address::generate(&env);
    client.add_admin(&Address::generate(&env));
    client.mock_all_auths().add_client(&new_client, &0);

    client.mock_all_auths().remove_client(&new_client);
}

#[test]
#[should_panic(expected = "El contrato no tiene un admin asignado")]
fn test_remove_client_no_admin() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let new_client = Address::generate(&env);
    client.add_client(&new_client, &0);

    client.remove_client(&new_client);
}

#[test]
#[should_panic(expected = "Cliente no encontrado")]
fn test_remove_client_not_found() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.add_admin(&admin);

    let new_client = Address::generate(&env);

    client.mock_all_auths().remove_client(&new_client);
}
