use hermes_cosmos_test_components::chain_driver::traits::{HasGrpcPort, HasRpcPort};
use hermes_prelude::*;
use hermes_relayer_components::chain::traits::{
    CanQueryBlock, HasBlockHash, HasChainId, HasGenesisHeight,
};
use hermes_runtime_components::traits::{
    CanReadFileAsString, CanReserveTcpPort, CanWriteStringToFile, HasRuntime,
};
use hermes_test_components::chain_driver::traits::{HasChain, HasChainType};
use hermes_test_components::driver::traits::HasChainDriverType;
use toml::Value;

use crate::bootstrap::traits::init_bridge_config::{
    BridgeConfigInitializer, BridgeConfigInitializerComponent,
};
use crate::bootstrap::traits::types::bridge_config::HasBridgeConfigType;
use crate::types::bridge_config::CelestiaBridgeConfig;

pub struct UpdateCelestiaBridgeConfig;

#[cgp_provider(BridgeConfigInitializerComponent)]
impl<Bootstrap, Runtime, Chain, ChainDriver> BridgeConfigInitializer<Bootstrap>
    for UpdateCelestiaBridgeConfig
where
    Bootstrap: HasRuntime<Runtime = Runtime>
        + HasChainType<Chain = Chain>
        + HasChainDriverType<ChainDriver = ChainDriver>
        + HasBridgeConfigType
        + CanRaiseAsyncError<Chain::Error>
        + CanRaiseAsyncError<Runtime::Error>
        + CanRaiseAsyncError<toml::de::Error>
        + CanRaiseAsyncError<toml::ser::Error>
        + CanRaiseAsyncError<&'static str>,
    Runtime: CanReadFileAsString + CanWriteStringToFile + CanReserveTcpPort,
    Chain: HasChainId + HasGenesisHeight + CanQueryBlock + HasBlockHash,
    ChainDriver: HasChain<Chain = Chain> + HasRpcPort + HasGrpcPort,
    Bootstrap::BridgeConfig: From<CelestiaBridgeConfig>,
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

        let block = chain
            .query_block(&chain.genesis_height())
            .await
            .map_err(Bootstrap::raise_error)?;

        let bridge_config_path = Runtime::join_file_path(
            bridge_home_dir,
            &Runtime::file_path_from_string(&format!(
                ".celestia-bridge-{chain_id_str}/config.toml"
            )),
        );

        let bridge_config_str = runtime
            .read_file_as_string(&bridge_config_path)
            .await
            .map_err(Bootstrap::raise_error)?;

        let node_rpc_port = chain_driver.rpc_port();
        let node_grpc_port = chain_driver.grpc_port();

        let mut bridge_config =
            toml::from_str(&bridge_config_str).map_err(Bootstrap::raise_error)?;

        set_trusted_hash(&mut bridge_config, &Chain::block_hash(&block).to_string())
            .map_err(Bootstrap::raise_error)?;

        set_chain_ip(&mut bridge_config, "127.0.0.1").map_err(Bootstrap::raise_error)?;

        set_chain_rpc_port(&mut bridge_config, node_rpc_port).map_err(Bootstrap::raise_error)?;

        set_chain_grpc_port(&mut bridge_config, node_grpc_port).map_err(Bootstrap::raise_error)?;

        let bridge_rpc_port = runtime
            .reserve_tcp_port()
            .await
            .map_err(Bootstrap::raise_error)?;

        set_bridge_rpc_port(&mut bridge_config, bridge_rpc_port).map_err(Bootstrap::raise_error)?;

        runtime
            .write_string_to_file(
                &bridge_config_path,
                &toml::to_string_pretty(&bridge_config).map_err(Bootstrap::raise_error)?,
            )
            .await
            .map_err(Bootstrap::raise_error)?;

        let config = CelestiaBridgeConfig {
            config: bridge_config,
            node_rpc_port,
            node_grpc_port,
            bridge_rpc_port,
        };

        Ok(config.into())
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

pub fn set_chain_rpc_port(config: &mut Value, rpc_port: u16) -> Result<(), &'static str> {
    config
        .get_mut("Core")
        .ok_or("expect header section")?
        .as_table_mut()
        .ok_or("expect object")?
        .insert("RPCPort".to_string(), rpc_port.to_string().into());

    Ok(())
}

pub fn set_chain_grpc_port(config: &mut Value, grpc_port: u16) -> Result<(), &'static str> {
    config
        .get_mut("Core")
        .ok_or("expect core section")?
        .as_table_mut()
        .ok_or("expect object")?
        .insert("GRPCPort".to_string(), grpc_port.to_string().into());

    Ok(())
}

pub fn set_bridge_rpc_port(config: &mut Value, rpc_port: u16) -> Result<(), &'static str> {
    config
        .get_mut("RPC")
        .ok_or("expect rpc section")?
        .as_table_mut()
        .ok_or("expect object")?
        .insert("Port".to_string(), rpc_port.to_string().into());

    Ok(())
}

pub fn set_chain_ip(config: &mut Value, ip: &str) -> Result<(), &'static str> {
    config
        .get_mut("Core")
        .ok_or("expect core section")?
        .as_table_mut()
        .ok_or("expect object")?
        .insert("GRPCPort".to_string(), ip.to_string().into());

    Ok(())
}
