use serde::Serialize;

#[derive(Serialize)]
pub struct SovereignGenesisConfig {
    pub accounts: AccountsGenesis,
    pub bank: BankGenesis,
    pub chain_state: ChainStateGenesis,
    pub sequencer_registry: SequencerRegistryGenesis,
}

#[derive(Serialize)]
pub struct AccountsGenesis {
    pub pub_keys: Vec<String>,
}

#[derive(Serialize)]
pub struct BankGenesis {
    pub tokens: Vec<TokenGenesis>,
}

#[derive(Serialize)]
pub struct TokenGenesis {
    pub token_name: String,
    pub address_and_balances: Vec<(String, u128)>,
    pub authorized_minters: Vec<String>,
    pub salt: u128,
}

#[derive(Serialize)]
pub struct ChainStateGenesis {
    pub initial_slot_height: u64,
    pub current_time: TimeGenesis,
    pub gas_price_blocks_depth: u64,
    pub gas_price_maximum_elasticity: u64,
    pub initial_gas_price: Vec<u64>,
    pub minimum_gas_price: Vec<u64>,
}

#[derive(Serialize)]
pub struct TimeGenesis {
    pub secs: u64,
    pub nanos: u32,
}

#[derive(Serialize)]
pub struct SequencerRegistryGenesis {
    pub seq_rollup_address: String,
    pub seq_da_address: String,
    pub coins_to_lock: CoinsToLock,
    pub is_preferred_sequencer: bool,
}

#[derive(Serialize)]
pub struct CoinsToLock {
    pub amount: u64,
    pub token_address: String,
}
