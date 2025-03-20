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

### 📆 Day 3 - Tokenización, deployar el contrato en Testnet
#### 🔍 Topicos a cubrir:
1. Tarea clase 2
2. Tokenización para depositar/retirar XLM
3. Deployar el contrato en la testnet

---

### ✅ Tarea clase 2:
Agregamos el recieve al contrato donde podemos leerlo, editarlo y eliminarlo.

#### 📂 **lib.rs**
```rust
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
```
#### 📂 **storage.rs**
```rust
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
```

#### 📂 **types.rs**
```rust
    #[derive(Clone)]
    #[contracttype]
    pub enum DataKey {
        Admin,           // Address
        Client(Address),
        Recieve(Address), // balance: i128
    }
```

### ✅ Tokenización para deposito/retiro:
Llevamos nuestro contrato al siguiente nivel agregando tokenización.
📥 deposit → El cliente agrega fondos al contrato.
📤 withdraw → El recieve retira los fondos del contrato.

#### 📂 **lib.rs**
```rust
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
```

#### 📂 **storage.rs**
```rust
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
```
#### 📂 **token.rs**
```rust
    use crate::storage;

    use soroban_sdk::{token, Address, Env};

    pub fn token_transfer(env: &Env, from: &Address, to: &Address, amount: &i128) {
        let token_address = storage::read_token(env);
        let token = token::TokenClient::new(env, &token_address);
        token.transfer(from, to, amount);
    }
```

#### 📂 **types.rs**
```rust
    #[derive(Clone)]
    #[contracttype]
    pub enum DataKey {
        Admin,           // Address
        Token,           // XLM Testnet Address
        ContractBalance, // i128
        Client(Address),
        Recieve(Address), // balance: i128
    }
```
### ✅ Deploy en Testnet:
🔑 Generar Keypair para las pruebas
  ```bash
  stellar keys generate --global alice --network testnet --fund
  ```


📌 Pasos para el deploy:
1️⃣ Compilar el contrato y generar el archivo .wasm
  ```bash
  stellar contract build
  ```
2️⃣ Deployar el contrato en la Testnet y obtener el contract ID
```bash
    stellar contract deploy `
        --wasm target/wasm32-unknown-unknown/release/hello_world.wasm `
        --source alice `
        --network testnet `
        --alias hello_world
  ```

🎉 ¡Felicidades! 🎉 Tu contrato ya está en la Testnet.
🔍 Verifícalo en [Stellar Expert](http://stellar.expert/explorer/testnet). 🚀💫

✅ 🌐 Inicializar el contrato desde la CLI

Una vez desplegado, inicializamos el contrato con el admin y la dirección del token.

1️⃣ Generar Admin Keypair para las pruebas
  ```bash
  stellar keys generate --global admin --network testnet --fund
  ```
2️⃣ Obtener el token address de XLM para usar en el contrato (CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC)
  ```bash
  stellar contract asset id --asset native --network testnet
  ```
3️⃣ Obtener el admin public key
  ```bash
  stellar keys address admin
  ```
4️ Iniciar el contrato con el admin y el token address de XLM
  ```bash
  stellar contract invoke `
  --id CCYVWS2NWHIQERFZOQZOL6UYYRLW2YLU2QHVMLFYNNXKYG5ZDSFNX77G `
  --source alice `
  --network testnet `
  -- `
  initialize `
  --admin GD6OU5CHTGFEZPYFX6TMLGMFVAHGHBLVTNHRUDDBYFKHQ55PHV2KZBI5
  `
  --token CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC
  ```

  🎉 ¡Felicidades! 🎉 Hemos concluido el Workshop subiendo un contrato a la red e iniciandolo para usarlo en nuestra DApp.