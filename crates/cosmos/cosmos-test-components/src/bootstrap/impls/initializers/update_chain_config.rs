use core::time::Duration;
use std::path::PathBuf;

use cgp::prelude::*;
use hermes_relayer_components::chain::traits::HasChainIdType;
use hermes_runtime_components::traits::fs::file_path::HasFilePathType;
use hermes_runtime_components::traits::fs::read_file::CanReadFileAsString;
use hermes_runtime_components::traits::fs::write_file::CanWriteStringToFile;
use hermes_runtime_components::traits::os::reserve_port::CanReserveTcpPort;
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_test_components::chain_driver::traits::HasChainType;
use ibc::core::host::types::identifiers::ChainId;
use toml::Value;

use crate::bootstrap::traits::initializers::init_chain_config::{
    ChainNodeConfigInitializer, ChainNodeConfigInitializerComponent,
};
use crate::bootstrap::traits::modifiers::modify_comet_config::CanModifyCometConfig;
use crate::bootstrap::traits::modifiers::modify_cosmos_sdk_config::CanModifyCosmosSdkConfig;
use crate::bootstrap::traits::types::chain_node_config::HasChainNodeConfigType;
use crate::bootstrap::traits::types::genesis_config::HasChainGenesisConfigType;
use crate::bootstrap::types::chain_node_config::CosmosChainNodeConfig;
use crate::bootstrap::types::genesis_config::CosmosGenesisConfig;

/// Parse the generated Comet and CosmosSDK TOML config files, and update the configuration
pub struct UpdateCosmosChainNodeConfig;

#[cgp_provider(ChainNodeConfigInitializerComponent)]
impl<Bootstrap, Runtime, Chain> ChainNodeConfigInitializer<Bootstrap>
    for UpdateCosmosChainNodeConfig
where
    Bootstrap: HasRuntime<Runtime = Runtime>
        + HasChainType<Chain = Chain>
        + HasChainNodeConfigType
        + HasChainGenesisConfigType<ChainGenesisConfig = CosmosGenesisConfig>
        + CanModifyCometConfig
        + CanModifyCosmosSdkConfig
        + CanRaiseAsyncError<Runtime::Error>
        + CanRaiseAsyncError<&'static str>
        + CanRaiseAsyncError<toml::de::Error>
        + CanRaiseAsyncError<toml::ser::Error>,
    Runtime: HasFilePathType<FilePath = PathBuf>
        + CanReadFileAsString
        + CanWriteStringToFile
        + CanReserveTcpPort,
    Bootstrap::ChainNodeConfig: From<CosmosChainNodeConfig>,
    Chain: HasChainIdType<ChainId = ChainId>,
{
    async fn init_chain_node_config(
        bootstrap: &Bootstrap,
        chain_home_dir: &PathBuf,
        chain_id: &ChainId,
        genesis_config: &CosmosGenesisConfig,
    ) -> Result<Bootstrap::ChainNodeConfig, Bootstrap::Error> {
        let runtime = bootstrap.runtime();

        let rpc_port = runtime
            .reserve_tcp_port()
            .await
            .map_err(Bootstrap::raise_error)?;

        let p2p_port = runtime
            .reserve_tcp_port()
            .await
            .map_err(Bootstrap::raise_error)?;

        let grpc_port = runtime
            .reserve_tcp_port()
            .await
            .map_err(Bootstrap::raise_error)?;

        let pprof_port = runtime
            .reserve_tcp_port()
            .await
            .map_err(Bootstrap::raise_error)?;

        let comet_config = {
            let comet_config_path = Runtime::join_file_path(
                chain_home_dir,
                &Runtime::file_path_from_string("config/config.toml"),
            );

            let mut comet_config = {
                let comet_config_string = runtime
                    .read_file_as_string(&comet_config_path)
                    .await
                    .map_err(Bootstrap::raise_error)?;

                toml::from_str(&comet_config_string).map_err(Bootstrap::raise_error)?
            };

            set_log_level(&mut comet_config, "info").map_err(Bootstrap::raise_error)?;

            set_rpc_port(&mut comet_config, rpc_port).map_err(Bootstrap::raise_error)?;

            set_p2p_port(&mut comet_config, p2p_port).map_err(Bootstrap::raise_error)?;

            set_pprof_port(&mut comet_config, pprof_port).map_err(Bootstrap::raise_error)?;

            set_timeout_commit(&mut comet_config, Duration::from_secs(1))
                .map_err(Bootstrap::raise_error)?;

            set_timeout_propose(&mut comet_config, Duration::from_secs(1))
                .map_err(Bootstrap::raise_error)?;
            set_mode(&mut comet_config, "validator").map_err(Bootstrap::raise_error)?;

            set_indexer(&mut comet_config, "kv").map_err(Bootstrap::raise_error)?;

            bootstrap.modify_comet_config(&mut comet_config)?;

            let comet_config_string =
                toml::to_string_pretty(&comet_config).map_err(Bootstrap::raise_error)?;

            runtime
                .write_string_to_file(&comet_config_path, &comet_config_string)
                .await
                .map_err(Bootstrap::raise_error)?;

            comet_config
        };

        let sdk_config = {
            let sdk_config_path = Runtime::join_file_path(
                chain_home_dir,
                &Runtime::file_path_from_string("config/app.toml"),
            );

            let mut sdk_config = {
                let sdk_config_string = runtime
                    .read_file_as_string(&sdk_config_path)
                    .await
                    .map_err(Bootstrap::raise_error)?;

                toml::from_str(&sdk_config_string).map_err(Bootstrap::raise_error)?
            };

            set_min_gas_price(
                &mut sdk_config,
                &format!("0{}", genesis_config.staking_denom),
            )
            .map_err(Bootstrap::raise_error)?;

            enable_grpc(&mut sdk_config).map_err(Bootstrap::raise_error)?;
            set_grpc_port(&mut sdk_config, grpc_port).map_err(Bootstrap::raise_error)?;
            disable_grpc_web(&mut sdk_config).map_err(Bootstrap::raise_error)?;
            disable_api(&mut sdk_config).map_err(Bootstrap::raise_error)?;

            bootstrap.modify_cosmos_sdk_config(&mut sdk_config)?;

            let sdk_config_string =
                toml::to_string_pretty(&sdk_config).map_err(Bootstrap::raise_error)?;

            runtime
                .write_string_to_file(&sdk_config_path, &sdk_config_string)
                .await
                .map_err(Bootstrap::raise_error)?;

            sdk_config
        };

        let chain_config = CosmosChainNodeConfig {
            chain_id: chain_id.clone(),
            chain_home_dir: chain_home_dir.clone(),
            rpc_port,
            p2p_port,
            pprof_port,
            grpc_port,
            comet_config,
            sdk_config,
        };

        Ok(chain_config.into())
    }
}

pub fn set_log_level(config: &mut Value, log_level: &str) -> Result<(), &'static str> {
    config
        .as_table_mut()
        .ok_or("expect object")?
        .insert("log_level".to_string(), log_level.into());

    Ok(())
}

pub fn set_rpc_port(config: &mut Value, port: u16) -> Result<(), &'static str> {
    config
        .get_mut("rpc")
        .ok_or("expect rpc section")?
        .as_table_mut()
        .ok_or("expect object")?
        .insert("laddr".to_string(), format!("tcp://0.0.0.0:{port}").into());

    Ok(())
}

pub fn set_p2p_port(config: &mut Value, port: u16) -> Result<(), &'static str> {
    config
        .get_mut("p2p")
        .ok_or("expect p2p section")?
        .as_table_mut()
        .ok_or("expect object")?
        .insert("laddr".to_string(), format!("tcp://0.0.0.0:{port}").into());

    Ok(())
}

pub fn set_pprof_port(config: &mut Value, port: u16) -> Result<(), &'static str> {
    config
        .get_mut("rpc")
        .ok_or("expect rpc section")?
        .as_table_mut()
        .ok_or("expect object")?
        .insert(
            "pprof_laddr".to_string(),
            format!("tcp://0.0.0.0:{port}").into(),
        );

    Ok(())
}

/// Set the `consensus.timeout_commit` field in the full node config.
pub fn set_timeout_commit(config: &mut Value, duration: Duration) -> Result<(), &'static str> {
    config
        .get_mut("consensus")
        .ok_or("expect consensus section")?
        .as_table_mut()
        .ok_or("expect object")?
        .insert(
            "timeout_commit".to_string(),
            format!("{}ms", duration.as_millis()).into(),
        );

    Ok(())
}

/// Set the `consensus.timeout_propose` field in the full node config.
pub fn set_timeout_propose(config: &mut Value, duration: Duration) -> Result<(), &'static str> {
    config
        .get_mut("consensus")
        .ok_or("expect consensus section")?
        .as_table_mut()
        .ok_or("expect object")?
        .insert(
            "timeout_propose".to_string(),
            format!("{}ms", duration.as_millis()).into(),
        );

    Ok(())
}

pub fn set_mode(config: &mut Value, mode: &str) -> Result<(), &'static str> {
    config
        .as_table_mut()
        .ok_or("expect object")?
        .insert("mode".to_string(), mode.into());

    Ok(())
}

pub fn set_indexer(config: &mut Value, mode: &str) -> Result<(), &'static str> {
    config
        .get_mut("tx_index")
        .ok_or("expect tx_index section")?
        .as_table_mut()
        .ok_or("expect object")?
        .insert("indexer".to_string(), mode.into());

    Ok(())
}

pub fn enable_grpc(config: &mut Value) -> Result<(), &'static str> {
    config
        .get_mut("grpc")
        .ok_or("expect grpc section")?
        .as_table_mut()
        .ok_or("expect object")?
        .insert("enable".to_string(), true.into());

    Ok(())
}

pub fn set_grpc_port(config: &mut Value, port: u16) -> Result<(), &'static str> {
    config
        .get_mut("grpc")
        .ok_or("expect grpc section")?
        .as_table_mut()
        .ok_or("expect object")?
        .insert("address".to_string(), format!("0.0.0.0:{port}").into());

    Ok(())
}

pub fn disable_grpc_web(config: &mut Value) -> Result<(), &'static str> {
    if let Some(field) = config.get_mut("grpc-web") {
        field
            .as_table_mut()
            .ok_or("expect object")?
            .insert("enable".to_string(), false.into());
    }

    Ok(())
}

pub fn disable_api(config: &mut Value) -> Result<(), &'static str> {
    if let Some(field) = config.get_mut("api") {
        field
            .as_table_mut()
            .ok_or("expect object")?
            .insert("enable".to_string(), false.into());
    }

    Ok(())
}

pub fn set_min_gas_price(config: &mut Value, value: &str) -> Result<(), &'static str> {
    config
        .as_table_mut()
        .ok_or("expect object")?
        .insert("minimum-gas-prices".into(), value.into());

    Ok(())
}
