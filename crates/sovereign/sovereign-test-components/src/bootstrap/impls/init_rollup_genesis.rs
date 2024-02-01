use cgp_core::CanRaiseError;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use hermes_test_components::runtime::traits::create_dir::CanCreateDir;
use hermes_test_components::runtime::traits::types::file_path::HasFilePathType;
use hermes_test_components::runtime::traits::write_file::CanWriteStringToFile;
use serde_json as json;

use crate::bootstrap::traits::init_rollup_genesis::RollupGenesisInitializer;
use crate::bootstrap::traits::types::rollup_genesis_config::HasRollupGenesisConfigType;
use crate::types::rollup_genesis_config::{
    AccountsGenesis, BankGenesis, ChainStateGenesis, CoinsToLock, SequencerRegistryGenesis,
    SovereignGenesisConfig, TimeGenesis,
};

pub struct InitSovereignGenesis;

impl<Bootstrap, Runtime> RollupGenesisInitializer<Bootstrap> for InitSovereignGenesis
where
    Bootstrap: HasRuntime<Runtime = Runtime>
        + HasRollupGenesisConfigType<RollupGenesisConfig = SovereignGenesisConfig>
        + CanRaiseError<Runtime::Error>
        + CanRaiseError<json::Error>,
    Runtime: HasFilePathType + CanWriteStringToFile + CanCreateDir,
{
    async fn init_rollup_genesis(
        bootstrap: &Bootstrap,
        rollup_home_dir: &Runtime::FilePath,
    ) -> Result<Bootstrap::RollupGenesisConfig, Bootstrap::Error> {
        let runtime = bootstrap.runtime();

        let genesis_dir =
            Runtime::join_file_path(rollup_home_dir, &Runtime::file_path_from_string("genesis"));

        runtime
            .create_dir(&genesis_dir)
            .await
            .map_err(Bootstrap::raise_error)?;

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

        {
            let account_genesis_path = Runtime::join_file_path(
                &genesis_dir,
                &Runtime::file_path_from_string("accounts.json"),
            );

            let account_genesis_str =
                json::to_string_pretty(&rollup_genesis.accounts).map_err(Bootstrap::raise_error)?;

            runtime
                .write_string_to_file(&account_genesis_path, &account_genesis_str)
                .await
                .map_err(Bootstrap::raise_error)?
        }

        {
            let bank_genesis_path =
                Runtime::join_file_path(&genesis_dir, &Runtime::file_path_from_string("bank.json"));

            let bank_genesis_str =
                json::to_string_pretty(&rollup_genesis.bank).map_err(Bootstrap::raise_error)?;

            runtime
                .write_string_to_file(&bank_genesis_path, &bank_genesis_str)
                .await
                .map_err(Bootstrap::raise_error)?
        }

        {
            let chain_state_genesis_path = Runtime::join_file_path(
                &genesis_dir,
                &Runtime::file_path_from_string("chain_state.json"),
            );

            let chain_state_genesis_str = json::to_string_pretty(&rollup_genesis.chain_state)
                .map_err(Bootstrap::raise_error)?;

            runtime
                .write_string_to_file(&chain_state_genesis_path, &chain_state_genesis_str)
                .await
                .map_err(Bootstrap::raise_error)?
        }

        {
            let sequencer_registry_genesis_path = Runtime::join_file_path(
                &genesis_dir,
                &Runtime::file_path_from_string("sequencer_registry.json"),
            );

            let sequencer_registry_genesis_str =
                json::to_string_pretty(&rollup_genesis.sequencer_registry)
                    .map_err(Bootstrap::raise_error)?;

            runtime
                .write_string_to_file(
                    &sequencer_registry_genesis_path,
                    &sequencer_registry_genesis_str,
                )
                .await
                .map_err(Bootstrap::raise_error)?
        }

        Ok(rollup_genesis)
    }
}
