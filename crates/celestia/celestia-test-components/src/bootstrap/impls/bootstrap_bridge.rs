use cgp_core::CanRaiseError;
use hermes_cosmos_test_components::bootstrap::types::chain_config::CosmosChainConfig;
use hermes_relayer_components::chain::traits::components::block_querier::CanQueryBlock;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainId;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use hermes_relayer_components::runtime::traits::sleep::CanSleep;
use hermes_test_components::chain_driver::traits::fields::chain_home_dir::HasChainHomeDir;
use hermes_test_components::chain_driver::traits::types::chain::{HasChain, HasChainType};
use hermes_test_components::driver::traits::types::chain_driver::HasChainDriverType;
use hermes_test_components::runtime::traits::child_process::CanStartChildProcess;
use hermes_test_components::runtime::traits::copy_file::CanCopyFile;
use hermes_test_components::runtime::traits::read_file::CanReadFileAsString;
use hermes_test_components::runtime::traits::types::child_process::HasChildProcessType;
use hermes_test_components::runtime::traits::types::file_path::HasFilePathType;
use hermes_test_components::runtime::traits::write_file::CanWriteStringToFile;
use ibc_relayer_types::core::ics02_client::error::Error as Ics02Error;
use ibc_relayer_types::core::ics24_host::identifier::ChainId;
use ibc_relayer_types::Height;
use tendermint::block::{Block, Id as BlockId};
use toml::Value;

use crate::bootstrap::traits::bootstrap_bridge::BridgeBootstrapper;
use crate::bootstrap::traits::bridge_store_dir::HasBridgeStoreDir;
use crate::bootstrap::traits::init_bridge_data::CanInitBridgeData;

pub struct BootstrapCelestiaBridge;

impl<Bootstrap, Chain, ChainDriver, Runtime> BridgeBootstrapper<Bootstrap>
    for BootstrapCelestiaBridge
where
    Bootstrap: HasChainType<Chain = Chain>
        + HasChainDriverType<ChainDriver = ChainDriver>
        + HasRuntime<Runtime = Runtime>
        + HasBridgeStoreDir
        + CanInitBridgeData
        + CanRaiseError<Chain::Error>
        + CanRaiseError<Runtime::Error>
        + CanRaiseError<Ics02Error>
        + CanRaiseError<toml::de::Error>
        + CanRaiseError<toml::ser::Error>
        + CanRaiseError<&'static str>,
    ChainDriver: HasChain<Chain = Chain> + HasRuntime<Runtime = Runtime> + HasChainHomeDir,
    Chain: HasChainId<ChainId = ChainId>
        + HasHeightType<Height = Height>
        + CanQueryBlock<Block = (BlockId, Block)>,
    Runtime: HasFilePathType
        + HasChildProcessType
        + CanCopyFile
        + CanStartChildProcess
        + CanSleep
        + CanReadFileAsString
        + CanWriteStringToFile,
{
    async fn bootstrap_bridge(
        boostrap: &Bootstrap,
        chain_driver: &ChainDriver,
        chain_config: &CosmosChainConfig,
    ) -> Result<Runtime::ChildProcess, Bootstrap::Error> {
        let runtime = boostrap.runtime();
        let chain = chain_driver.chain();
        let chain_home_dir = chain_driver.chain_home_dir();

        let chain_id = chain.chain_id();
        let chain_id_str = chain_id.to_string();
        let bridge_store_dir = boostrap.bridge_store_dir();

        let bridge_home_dir = Runtime::join_file_path(
            bridge_store_dir,
            &Runtime::file_path_from_string(&chain_id_str),
        );

        boostrap
            .init_bridge_data(&bridge_home_dir, chain_id)
            .await?;

        let bridge_key_source_path = Runtime::join_file_path(
            chain_home_dir,
            &Runtime::file_path_from_string("keyring-test/bridge.info"),
        );

        let bridge_key_destination_path = Runtime::join_file_path(
            &bridge_home_dir,
            &Runtime::file_path_from_string(&format!(
                ".celestia-bridge-{chain_id_str}/keys/keyring-test/bridge.info"
            )),
        );

        runtime
            .copy_file(&bridge_key_source_path, &bridge_key_destination_path)
            .await
            .map_err(Bootstrap::raise_error)?;

        let genesis_height =
            Height::new(chain.chain_id().version(), 1).map_err(Bootstrap::raise_error)?;

        let (block_id, _) = chain
            .query_block(&genesis_height)
            .await
            .map_err(Bootstrap::raise_error)?;

        let block_hash = block_id.hash;

        let bridge_config_path = Runtime::join_file_path(
            &bridge_home_dir,
            &Runtime::file_path_from_string(&format!(
                ".celestia-bridge-{chain_id_str}/config.toml"
            )),
        );

        let bridge_config_str = runtime
            .read_file_as_string(&bridge_config_path)
            .await
            .map_err(Bootstrap::raise_error)?;

        let mut bridge_config =
            toml::from_str(&bridge_config_str).map_err(Bootstrap::raise_error)?;

        set_trusted_hash(&mut bridge_config, &block_hash.to_string())
            .map_err(Bootstrap::raise_error)?;

        runtime
            .write_string_to_file(
                &bridge_config_path,
                &toml::to_string_pretty(&bridge_config).map_err(Bootstrap::raise_error)?,
            )
            .await
            .map_err(Bootstrap::raise_error)?;

        let stdout_path = Runtime::join_file_path(
            &bridge_home_dir,
            &Runtime::file_path_from_string("stdout.log"),
        );

        let stderr_path = Runtime::join_file_path(
            &bridge_home_dir,
            &Runtime::file_path_from_string("stderr.log"),
        );

        let child = runtime
            .start_child_process(
                &Runtime::file_path_from_string("celestia"),
                &[
                    "bridge",
                    "start",
                    "--keyring.accname",
                    "bridge",
                    "--core.ip",
                    "127.0.0.1",
                    "--core.rpc.port",
                    &chain_config.rpc_port.to_string(),
                    "--core.grpc.port",
                    &chain_config.grpc_port.to_string(),
                    "--p2p.network",
                    &chain_id_str,
                ],
                &[("HOME", &Runtime::file_path_to_string(&bridge_home_dir))],
                Some(&stdout_path),
                Some(&stderr_path),
            )
            .await
            .map_err(Bootstrap::raise_error)?;

        Ok(child)
    }
}

pub fn set_trusted_hash(config: &mut Value, trusted_hash: &str) -> Result<(), &'static str> {
    config
        .get_mut("Header")
        .ok_or("expect header section")?
        .as_table_mut()
        .ok_or("expect object")?
        .insert("TrustedHash".to_string(), trusted_hash.into());

    Ok(())
}
