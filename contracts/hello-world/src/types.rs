use soroban_sdk::{contracttype, Address};

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
