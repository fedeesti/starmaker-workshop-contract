# Soroban + Rust Workshop for Blockchain Hackathon

## 📅 Workshop Overview
Este taller es un **bootcamp de 3 días (2 horas por día)** centrado en enseñar a los participantes a construir contratos inteligentes con **Rust y Soroban**. Al final del taller, los participantes tendrán las habilidades para crear, probar y desplegar sus propios contratos inteligentes utilizando Soroban.

### 📆 Day 1 (Completado)
- Introducción a Rust
- Crear un Smart Contract básico:
  ```bash
  stellar contract init soroban-hello-world
  ```

---

### 📆 Day 2 - Storage, Structs y Testing
#### 🔍 Topicos a cubrir:
1. Storage y Persistencia
2. Structs para almacenamiento
3. Testeo de los nuevos métodos implementados

---

### ✅ Example 1: Basic Storage
Crearemos un contrato simple que almacene y recupere el address del admin del contrato.

#### 📂 **lib.rs**
```rust
use soroban_sdk::{Env, Address};

    pub fn add_admin(env: Env, admin: Address) {
        if storage::has_admin(&env) {
            panic!("El admin ya ha sido asignado");
        }
        storage::write_admin(&env, &admin);
    }

    pub fn get_admin(env: Env) -> Address {
        storage::read_admin(&env)
    }
```

#### 📂 **storage.rs**
```rust
use soroban_sdk::{Address, Env};

use crate::types::DataKey;

pub fn has_admin(env: &Env) -> bool {
    env.storage().persistent().has(&DataKey::Admin)
}

pub fn read_admin(env: &Env) -> Address {
    env.storage().persistent().get(&DataKey::Admin).unwrap()
}

pub fn write_admin(env: &Env, admin: &Address) {
    env.storage().persistent().set(&DataKey::Admin, admin);
}

```

#### 📂 **types.rs**
```rust
use soroban_sdk::contracttype;

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin, // Address
}
```

#### 📂 **types.rs**
```rust
#[test]
fn test_add_admin() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.add_admin(&admin);
    assert_eq!(client.get_admin(), admin);
}

#[test]
#[should_panic(expected = "El admin ya ha sido asignado")]
fn test_add_admin_twice() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.add_admin(&admin);
    assert_eq!(client.get_admin(), admin);

    let admin2 = Address::generate(&env);
    client.add_admin(&admin2);
}
```

Buildeamos el contrato y corremos los test
```
cargo build --target wasm32-unknown-unknown --release
cargo test
```
---

### ✅ Example 2: Almacenamiento de Struct
Vamos a usar structs y enum para el almacenamiento

#### 📂 **lib.rs**
```rust
    fn check_admin(env: Env) {
        if !storage::has_admin(&env) {
            panic!("El contrato no tiene un admin asignado");
        }

        let admin = storage::read_admin(&env);

        admin.require_auth();
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
```

#### 📂 **storage.rs**
```rust
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
```

#### 📂 **types.rs**
```rust
#[derive(Clone, PartialEq, Debug)]
#[contracttype]
#[repr(u32)]
pub enum ClientStatus {
    Enabled,  // 0
    Disabled, // 1
}

#[derive(Clone)]
#[contracttype]
pub struct Client {
    pub balance: i128,
    pub status: ClientStatus,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin, // Address
    Client(Address),
}
```

#### 📂 **test.rs**
```rust
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

```

Buildeamos el contrato y corremos los test
```
cargo build --target wasm32-unknown-unknown --release
cargo test
```

## 🏆 Actividades y Tareas

### 🎯 Actividades para la Casa

#### 🔹 Tarea 1: Agregar datos a la struct del Cliente

📌 Descripción:
Agregar un nuevo tipo de usuario llamado Recieve que va a recibir dinero del cliente.

📌 Requisitos:

- Agregar en el almacenamiento el Recieve 
- Agregar al storage que pueda leer, obtener, escribir y remover al Recieve

#### 🔹 Tarea Avanzada:

📌 Descripción:
Un cliente debe enviar dinero al recieve. Aclaración: se debe descontar al cliente el monto que va a enviar y el recieve aumentar su balance.

📌 Requisitos:

- Agregar una función `balance` para el envío de dinero
- Agregar test para corroborar que el envío funciona correctamente

