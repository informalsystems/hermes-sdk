use cgp_core::CanRaiseError;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use serde_json as json;

use crate::bootstrap::traits::generate_rollup_genesis::RollupGenesisGenerator;
use crate::bootstrap::traits::types::rollup_genesis_config::HasRollupGenesisConfigType;
use crate::types::rollup_genesis_config::{
    AccountsGenesis, BankGenesis, ChainStateGenesis, CoinsToLock, SequencerRegistryGenesis,
    SovereignGenesisConfig, TimeGenesis,
};

pub struct GenerateSovereignGenesis;

impl<Bootstrap, Runtime> RollupGenesisGenerator<Bootstrap> for GenerateSovereignGenesis
where
    Bootstrap: HasRuntime<Runtime = Runtime>
        + HasRollupGenesisConfigType<RollupGenesisConfig = SovereignGenesisConfig>
        + CanRaiseError<json::Error>,
{
    async fn generate_rollup_genesis(
        _bootstrap: &Bootstrap,
    ) -> Result<Bootstrap::RollupGenesisConfig, Bootstrap::Error> {
        let rollup_genesis = SovereignGenesisConfig {
            accounts: AccountsGenesis { pub_keys: vec![] },
            bank: BankGenesis { tokens: vec![] },
            chain_state: ChainStateGenesis {
                current_time: TimeGenesis { secs: 0, nanos: 0 },
                gas_price_blocks_depth: 0,
                gas_price_maximum_elasticity: 0,
                initial_gas_price: vec![0],
                minimum_gas_price: vec![0],
            },
            sequencer_registry: SequencerRegistryGenesis {
                seq_rollup_address: "".to_owned(),
                seq_da_address: "".to_owned(),
                coins_to_lock: CoinsToLock {
                    amount: 0,
                    token_address: "".to_owned(),
                },
                is_preferred_sequencer: true,
            },
        };

        Ok(rollup_genesis)
    }
}
