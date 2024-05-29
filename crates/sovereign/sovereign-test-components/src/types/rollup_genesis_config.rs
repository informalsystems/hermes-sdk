use hermes_sovereign_rollup_components::types::address::SovereignAddress;
use serde::Serialize;

pub struct SovereignGenesisConfig {
    pub accounts: AccountsGenesis,
    pub bank: BankGenesis,
    pub chain_state: ChainStateGenesis,
    pub sequencer_registry: SequencerRegistryGenesis,
    pub prover_incentives: ProverIncentivesGenesis,
    pub staking_token_address: SovereignAddress,
    pub transfer_token_address: SovereignAddress,
}

#[derive(Serialize)]
pub struct AccountsGenesis {
    pub accounts: Vec<AccountGenesis>,
}

#[derive(Serialize)]
pub struct AccountGenesis {
    pub credential_id: String,
    pub address: String,
}

#[derive(Serialize)]
pub struct BankGenesis {
    pub gas_token_config: TokenGenesis,
    pub tokens: Vec<TokenGenesis>,
}

#[derive(Serialize)]
pub struct TokenGenesis {
    pub token_name: String,
    pub token_id: String,
    pub address_and_balances: Vec<(String, u128)>,
    pub authorized_minters: Vec<String>,
    pub salt: u128,
}

#[derive(Serialize)]
pub struct ChainStateGenesis {
    pub current_time: TimeGenesis,
    pub genesis_da_height: u64,
    pub inner_code_commitment: [u8; 8],
    pub outer_code_commitment: [u8; 32],
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
    pub minimum_bond: u64,
    pub is_preferred_sequencer: bool,
}

#[derive(Serialize)]
pub struct ProverIncentivesGenesis {
    pub proving_penalty: u64,
    pub minimum_bond: u64,
    pub initial_provers: Vec<(String, u64)>,
}
