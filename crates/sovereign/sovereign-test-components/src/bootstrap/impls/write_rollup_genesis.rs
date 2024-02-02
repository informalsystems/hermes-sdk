use cgp_core::CanRaiseError;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use hermes_test_components::runtime::traits::create_dir::CanCreateDir;
use hermes_test_components::runtime::traits::types::file_path::HasFilePathType;
use hermes_test_components::runtime::traits::write_file::CanWriteStringToFile;
use serde_json as json;

use crate::bootstrap::traits::types::rollup_genesis_config::HasRollupGenesisConfigType;
use crate::bootstrap::traits::write_rollup_genesis::RollupGenesisWriter;
use crate::types::rollup_genesis_config::SovereignGenesisConfig;

pub struct WriteSovereignGenesis;

impl<Bootstrap, Runtime> RollupGenesisWriter<Bootstrap> for WriteSovereignGenesis
where
    Bootstrap: HasRuntime<Runtime = Runtime>
        + HasRollupGenesisConfigType<RollupGenesisConfig = SovereignGenesisConfig>
        + CanRaiseError<Runtime::Error>
        + CanRaiseError<json::Error>,
    Runtime: HasFilePathType + CanWriteStringToFile + CanCreateDir,
{
    async fn write_rollup_genesis(
        bootstrap: &Bootstrap,
        rollup_home_dir: &Runtime::FilePath,
        genesis_config: &SovereignGenesisConfig,
    ) -> Result<(), Bootstrap::Error> {
        let runtime = bootstrap.runtime();

        let genesis_dir =
            Runtime::join_file_path(rollup_home_dir, &Runtime::file_path_from_string("genesis"));

        runtime
            .create_dir(&genesis_dir)
            .await
            .map_err(Bootstrap::raise_error)?;

        {
            let account_genesis_path = Runtime::join_file_path(
                &genesis_dir,
                &Runtime::file_path_from_string("accounts.json"),
            );

            let account_genesis_str =
                json::to_string_pretty(&genesis_config.accounts).map_err(Bootstrap::raise_error)?;

            runtime
                .write_string_to_file(&account_genesis_path, &account_genesis_str)
                .await
                .map_err(Bootstrap::raise_error)?
        }

        {
            let bank_genesis_path =
                Runtime::join_file_path(&genesis_dir, &Runtime::file_path_from_string("bank.json"));

            let bank_genesis_str =
                json::to_string_pretty(&genesis_config.bank).map_err(Bootstrap::raise_error)?;

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

            let chain_state_genesis_str = json::to_string_pretty(&genesis_config.chain_state)
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
                json::to_string_pretty(&genesis_config.sequencer_registry)
                    .map_err(Bootstrap::raise_error)?;

            runtime
                .write_string_to_file(
                    &sequencer_registry_genesis_path,
                    &sequencer_registry_genesis_str,
                )
                .await
                .map_err(Bootstrap::raise_error)?
        }

        Ok(())
    }
}
