#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Env};

#[test]
fn test_initialize() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let token = Address::generate(&env);
    client.initialize(&admin, &token);
    assert_eq!(client.get_admin(), admin);
}

#[test]
#[should_panic(expected = "El contrato ya ha sido inicializado")]
fn test_initialize_twice() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let token = Address::generate(&env);
    client.initialize(&admin, &token);
    assert_eq!(client.get_admin(), admin);

    let admin2 = Address::generate(&env);
    let token2 = Address::generate(&env);
    client.initialize(&admin2, &token2);
}

#[test]
fn test_add_client() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let token = Address::generate(&env);
    client.initialize(&admin, &token);

    let new_client = Address::generate(&env);

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

    let admin = Address::generate(&env);
    let token = Address::generate(&env);
    client.initialize(&admin, &token);

    let new_client = Address::generate(&env);
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
    let token = Address::generate(&env);
    client.initialize(&admin, &token);

    let new_client = Address::generate(&env);

    client.mock_all_auths().update_client(&new_client, &true);
}

#[test]
fn test_remove_client() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let token = Address::generate(&env);
    client.initialize(&admin, &token);

    let new_client = Address::generate(&env);
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
    let token = Address::generate(&env);
    client.initialize(&admin, &token);

    let new_client = Address::generate(&env);

    client.mock_all_auths().remove_client(&new_client);
}

#[test]
fn test_add_recieve() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let token = Address::generate(&env);
    client.initialize(&admin, &token);

    let new_recieve = Address::generate(&env);

    client.mock_all_auths().add_recieve(&new_recieve, &0);
    assert_eq!(client.amount_to_withdraw(&new_recieve), 0i128);
}

#[test]
#[should_panic(expected = "El contrato no tiene un admin asignado")]
fn test_add_recieve_without_admin() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let new_recieve = Address::generate(&env);

    client.mock_all_auths().add_recieve(&new_recieve, &0);
}

#[test]
fn test_remove_recieve() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let token = Address::generate(&env);
    client.initialize(&admin, &token);

    let new_recieve = Address::generate(&env);

    client.mock_all_auths().add_recieve(&new_recieve, &0);
    assert_eq!(client.amount_to_withdraw(&new_recieve), 0i128);

    client.mock_all_auths().remove_recieve(&new_recieve);
}

#[test]
#[should_panic(expected = "El contrato no tiene un admin asignado")]
fn test_remove_recieve_without_admin() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let new_recieve = Address::generate(&env);

    client.mock_all_auths().remove_recieve(&new_recieve);
}

#[test]
#[should_panic(expected = "Recieve no encontrado")]
fn test_remove_recieve_not_found() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let token = Address::generate(&env);
    client.initialize(&admin, &token);

    let new_recieve = Address::generate(&env);

    client.mock_all_auths().remove_recieve(&new_recieve);
}

#[test]
#[should_panic(expected = "Cliente no encontrado")]
fn test_deposit_without_client() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let token = Address::generate(&env);
    client.initialize(&admin, &token);

    client.deposit(&admin, &token, &10i128);
}

#[test]
#[should_panic(expected = "Recieve no encontrado")]
fn test_deposit_without_recieve() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let token = Address::generate(&env);
    client.initialize(&admin, &token);

    let new_client = Address::generate(&env);
    client.mock_all_auths().add_client(&new_client, &0);

    client.deposit(&new_client, &admin, &10i128);
}

#[test]
#[should_panic(expected = "Recieve no encontrado")]
fn test_withdraw_without_recieve() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let token = Address::generate(&env);
    client.initialize(&admin, &token);

    let new_client = Address::generate(&env);
    client.mock_all_auths().add_client(&new_client, &0);

    client.withdraw(&token, &8i128);
}
