use alloc::sync::Arc;
use hermes_cosmos_chain_components::types::config::gas::dynamic_gas_config::DynamicGasConfig;
use hermes_error::types::Error;
use serde_json::Value as JsonValue;
use std::env;
use toml::Value as TomlValue;

use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
use hermes_runtime::types::runtime::HermesRuntime;
use tokio::runtime::Builder;
use tracing::info;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter};

use crate::contexts::bootstrap::CosmosBootstrap;

pub fn init_test_runtime() -> HermesRuntime {
    let _ = stable_eyre::install();

    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(env_filter)
        .init();

    let tokio_runtime = Arc::new(Builder::new_multi_thread().enable_all().build().unwrap());

    let runtime = HermesRuntime::new(tokio_runtime.clone());

    info!("initialized Hermes test runtime");

    runtime
}

pub fn init_bootstrap(
    chain_id: usize,
    runtime: HermesRuntime,
    should_randomize_identifiers: bool,
    chain_store_dir: &str,
    transfer_denom_prefix: String,
    genesis_modifier: impl Fn(&mut JsonValue) -> Result<(), Error> + Send + Sync + 'static,
    comet_config_modifier: impl Fn(&mut TomlValue) -> Result<(), Error> + Send + Sync + 'static,
    dynamic_gas: Option<DynamicGasConfig>,
) -> CosmosBootstrap {
    let cosmos_builder = Arc::new(CosmosBuilder::new_with_default(runtime.clone()));

    let chain_command_path =
        env::var("CHAIN_COMMAND_PATHS").unwrap_or_else(|_| "gaiad".to_string());
    let chain_command_paths: Vec<String> = parse_chain_command_paths(chain_command_path);

    let account_prefix = env::var("ACCOUNT_PREFIXES").unwrap_or_else(|_| "cosmos".to_string());
    let account_prefixes = parse_chain_command_paths(account_prefix);

    let staking_denom_prefix = env::var("NATIVE_TOKENS").unwrap_or_else(|_| "stake".to_string());
    let staking_denom_prefixes = parse_chain_command_paths(staking_denom_prefix);

    CosmosBootstrap {
        runtime,
        cosmos_builder,
        should_randomize_identifiers,
        chain_store_dir: chain_store_dir.into(),
        chain_command_path: chain_command_paths[chain_id % chain_command_paths.len()]
            .as_str()
            .into(),
        account_prefix: account_prefixes[chain_id % account_prefixes.len()]
            .as_str()
            .into(),
        staking_denom_prefix: staking_denom_prefixes[chain_id % staking_denom_prefixes.len()]
            .as_str()
            .into(),
        transfer_denom_prefix,
        genesis_config_modifier: Box::new(genesis_modifier),
        comet_config_modifier: Box::new(comet_config_modifier),
        dynamic_gas,
    }
}

fn parse_chain_command_paths(chain_command_path: String) -> Vec<String> {
    let patterns: Vec<String> = chain_command_path
        .split(',')
        .map(|chain_binary| chain_binary.to_string())
        .collect();
    patterns
}
