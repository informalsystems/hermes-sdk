use cgp_core::CanRaiseError;
use hermes_relayer_components::chain::traits::components::block_querier::CanQueryBlock;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainId;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use hermes_test_components::chain_driver::traits::types::chain::{HasChain, HasChainType};
use hermes_test_components::driver::traits::types::chain_driver::HasChainDriverType;
use hermes_test_components::runtime::traits::read_file::CanReadFileAsString;
use hermes_test_components::runtime::traits::write_file::CanWriteStringToFile;
use ibc_relayer_types::core::ics02_client::error::Error as Ics02Error;
use ibc_relayer_types::core::ics24_host::identifier::ChainId;
use ibc_relayer_types::Height;
use tendermint::block::{Block, Id as BlockId};
use toml::Value;

use crate::bootstrap::traits::init_bridge_config::BridgeConfigInitializer;
use crate::bootstrap::traits::types::bridge_config::HasBridgeConfigType;

pub struct UpdateCelestiaBridgeConfig;

impl<Bootstrap, Runtime, Chain, ChainDriver> BridgeConfigInitializer<Bootstrap>
    for UpdateCelestiaBridgeConfig
where
    Bootstrap: HasRuntime<Runtime = Runtime>
        + HasChainType<Chain = Chain>
        + HasChainDriverType<ChainDriver = ChainDriver>
        + HasBridgeConfigType
        + CanRaiseError<Chain::Error>
        + CanRaiseError<Runtime::Error>
        + CanRaiseError<Ics02Error>
        + CanRaiseError<toml::de::Error>
        + CanRaiseError<toml::ser::Error>
        + CanRaiseError<&'static str>,
    Runtime: CanReadFileAsString + CanWriteStringToFile,
    Chain: HasChainId<ChainId = ChainId>
        + HasHeightType<Height = Height>
        + CanQueryBlock<Block = (BlockId, Block)>,
    ChainDriver: HasChain<Chain = Chain>,
    Bootstrap::BridgeConfig: From<Value>,
{
    async fn init_bridge_config(
        bootstrap: &Bootstrap,
        bridge_home_dir: &Runtime::FilePath,
        chain_driver: &ChainDriver,
    ) -> Result<Bootstrap::BridgeConfig, Bootstrap::Error> {
        let runtime = bootstrap.runtime();
        let chain = chain_driver.chain();
        let chain_id = chain.chain_id();
        let chain_id_str = chain_id.to_string();

        let genesis_height = Height::new(chain_id.version(), 1).map_err(Bootstrap::raise_error)?;

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

        Ok(bridge_config.into())
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
