use core::time::Duration;

use cgp_core::prelude::*;
use eyre::{eyre, Report};
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use hermes_test_components::runtime::traits::read_file::CanReadFileAsString;
use hermes_test_components::runtime::traits::reserve_port::CanReserveTcpPort;
use hermes_test_components::runtime::traits::types::file_path::HasFilePathType;
use hermes_test_components::runtime::traits::write_file::CanWriteStringToFile;
use toml::Value;

use crate::bootstrap::traits::initializers::init_chain_config::ChainConfigInitializer;
use crate::bootstrap::traits::modifiers::modify_comet_config::CanModifyCometConfig;
use crate::bootstrap::traits::types::chain_config::HasChainConfigType;
use crate::bootstrap::types::chain_config::CosmosChainConfig;

/// Parse the generated Comet and CosmosSDK TOML config files, and update the configuration
pub struct UpdateCosmosChainConfig;

#[async_trait]
impl<Bootstrap, Runtime> ChainConfigInitializer<Bootstrap> for UpdateCosmosChainConfig
where
    Bootstrap:
        HasRuntime<Runtime = Runtime> + HasErrorType + HasChainConfigType + CanModifyCometConfig,
    Runtime: HasFilePathType + CanReadFileAsString + CanWriteStringToFile + CanReserveTcpPort,
    Bootstrap::Error: From<Report>,
    Bootstrap::ChainConfig: From<CosmosChainConfig>,
{
    async fn init_chain_config(
        bootstrap: &Bootstrap,
        chain_home_dir: &Runtime::FilePath,
    ) -> Result<Bootstrap::ChainConfig, Bootstrap::Error> {
        let runtime = bootstrap.runtime();

        let rpc_port = runtime
            .reserve_tcp_port()
            .await
            .map_err(Bootstrap::raise_error)?;
        let p2p_port = runtime
            .reserve_tcp_port()
            .await
            .map_err(Bootstrap::raise_error)?;
        let pprof_port = runtime
            .reserve_tcp_port()
            .await
            .map_err(Bootstrap::raise_error)?;
        let grpc_port = runtime
            .reserve_tcp_port()
            .await
            .map_err(Bootstrap::raise_error)?;

        let comet_config = {
            let mut comet_config = {
                let comet_config_path = Runtime::join_file_path(
                    chain_home_dir,
                    &Runtime::file_path_from_string("config/config.toml"),
                );

                let comet_config_string = runtime
                    .read_file_as_string(&comet_config_path)
                    .await
                    .map_err(Bootstrap::raise_error)?;

                toml::from_str(&comet_config_string).map_err(Report::from)?
            };

            set_log_level(&mut comet_config, "info")?;
            set_rpc_port(&mut comet_config, rpc_port)?;
            set_p2p_port(&mut comet_config, p2p_port)?;
            set_pprof_port(&mut comet_config, pprof_port)?;
            set_timeout_commit(&mut comet_config, Duration::from_secs(1))?;
            set_timeout_propose(&mut comet_config, Duration::from_secs(1))?;
            set_mode(&mut comet_config, "validator")?;
            set_indexer(&mut comet_config, "kv")?;

            bootstrap.modify_comet_config(&mut comet_config)?;

            comet_config
        };

        let sdk_config = {
            let mut sdk_config = {
                let sdk_config_path = Runtime::join_file_path(
                    chain_home_dir,
                    &Runtime::file_path_from_string("config/app.toml"),
                );

                let sdk_config_string = runtime
                    .read_file_as_string(&sdk_config_path)
                    .await
                    .map_err(Bootstrap::raise_error)?;

                toml::from_str(&sdk_config_string).map_err(Report::from)?
            };

            set_grpc_port(&mut sdk_config, grpc_port)?;
            disable_grpc_web(&mut sdk_config)?;
            disable_api(&mut sdk_config)?;

            sdk_config
        };

        let chain_config = CosmosChainConfig {
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

pub fn set_log_level(config: &mut Value, log_level: &str) -> Result<(), Report> {
    config
        .as_table_mut()
        .ok_or_else(|| eyre!("expect object"))?
        .insert("log_level".to_string(), log_level.into());

    Ok(())
}

pub fn set_rpc_port(config: &mut Value, port: u16) -> Result<(), Report> {
    config
        .get_mut("rpc")
        .ok_or_else(|| eyre!("expect rpc section"))?
        .as_table_mut()
        .ok_or_else(|| eyre!("expect object"))?
        .insert("laddr".to_string(), format!("tcp://0.0.0.0:{port}").into());

    Ok(())
}

pub fn set_p2p_port(config: &mut Value, port: u16) -> Result<(), Report> {
    config
        .get_mut("p2p")
        .ok_or_else(|| eyre!("expect p2p section"))?
        .as_table_mut()
        .ok_or_else(|| eyre!("expect object"))?
        .insert("laddr".to_string(), format!("tcp://0.0.0.0:{port}").into());

    Ok(())
}

pub fn set_pprof_port(config: &mut Value, port: u16) -> Result<(), Report> {
    config
        .as_table_mut()
        .ok_or_else(|| eyre!("expect object"))?
        .insert(
            "pprof_laddr".to_string(),
            format!("tcp://0.0.0.0:{port}").into(),
        );

    Ok(())
}

/// Set the `consensus.timeout_commit` field in the full node config.
pub fn set_timeout_commit(config: &mut Value, duration: Duration) -> Result<(), Report> {
    config
        .get_mut("consensus")
        .ok_or_else(|| eyre!("expect consensus section"))?
        .as_table_mut()
        .ok_or_else(|| eyre!("expect object"))?
        .insert(
            "timeout_commit".to_string(),
            format!("{}ms", duration.as_millis()).into(),
        );

    Ok(())
}

/// Set the `consensus.timeout_propose` field in the full node config.
pub fn set_timeout_propose(config: &mut Value, duration: Duration) -> Result<(), Report> {
    config
        .get_mut("consensus")
        .ok_or_else(|| eyre!("expect consensus section"))?
        .as_table_mut()
        .ok_or_else(|| eyre!("expect object"))?
        .insert(
            "timeout_propose".to_string(),
            format!("{}ms", duration.as_millis()).into(),
        );

    Ok(())
}

pub fn set_mode(config: &mut Value, mode: &str) -> Result<(), Report> {
    config
        .as_table_mut()
        .ok_or_else(|| eyre!("expect object"))?
        .insert("mode".to_string(), mode.into());

    Ok(())
}

pub fn set_indexer(config: &mut Value, mode: &str) -> Result<(), Report> {
    config
        .get_mut("tx_index")
        .ok_or_else(|| eyre!("expect tx_index section"))?
        .as_table_mut()
        .ok_or_else(|| eyre!("expect object"))?
        .insert("indexer".to_string(), mode.into());

    Ok(())
}

pub fn set_grpc_port(config: &mut Value, port: u16) -> Result<(), Report> {
    config
        .get_mut("grpc")
        .ok_or_else(|| eyre!("expect grpc section"))?
        .as_table_mut()
        .ok_or_else(|| eyre!("expect object"))?
        .insert("address".to_string(), format!("0.0.0.0:{port}").into());

    Ok(())
}

pub fn disable_grpc_web(config: &mut Value) -> Result<(), Report> {
    if let Some(field) = config.get_mut("grpc-web") {
        field
            .as_table_mut()
            .ok_or_else(|| eyre!("expect object"))?
            .insert("enable".to_string(), false.into());
    }

    Ok(())
}

pub fn disable_api(config: &mut Value) -> Result<(), Report> {
    if let Some(field) = config.get_mut("api") {
        field
            .as_table_mut()
            .ok_or_else(|| eyre!("expect object"))?
            .insert("enable".to_string(), false.into());
    }

    Ok(())
}
