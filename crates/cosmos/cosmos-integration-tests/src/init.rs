use alloc::sync::Arc;
use eyre::Report;
use serde_json::Value as JsonValue;
use std::env;
use std::str::FromStr;
use tokio::runtime::Builder;
use toml::Value as TomlValue;
use tracing::info;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter};

use hermes_cosmos_chain_components::types::config::gas::dynamic_gas_config::DynamicGasConfig;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
use hermes_error::types::Error;
use hermes_runtime::types::runtime::HermesRuntime;

use crate::contexts::bootstrap::CosmosBootstrap;
use crate::contexts::bootstrap_legacy::LegacyCosmosBootstrap;

pub enum TestPreset {
    CosmosToCosmos,
}

impl FromStr for TestPreset {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "cosmostocosmos" => Ok(TestPreset::CosmosToCosmos),
            _ => Err(Report::msg("unknown test preset: `{s}`")),
        }
    }
}

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

pub fn build_osmosis_bootstrap(
    runtime: HermesRuntime,
    should_randomize_identifiers: bool,
    chain_store_dir: &str,
    transfer_denom_prefix: String,
    genesis_modifier: impl Fn(&mut JsonValue) -> Result<(), Error> + Send + Sync + 'static,
    comet_config_modifier: impl Fn(&mut TomlValue) -> Result<(), Error> + Send + Sync + 'static,
) -> LegacyCosmosBootstrap {
    let cosmos_builder = Arc::new(CosmosBuilder::new_with_default(runtime.clone()));

    LegacyCosmosBootstrap {
        runtime,
        cosmos_builder,
        should_randomize_identifiers,
        chain_store_dir: chain_store_dir.into(),
        chain_command_path: "osmosisd".into(),
        account_prefix: "osmosis".into(),
        compat_mode: None,
        staking_denom_prefix: "stake".into(),
        transfer_denom_prefix,
        genesis_config_modifier: Box::new(genesis_modifier),
        comet_config_modifier: Box::new(comet_config_modifier),
    }
}

pub fn build_gaia_bootstrap(
    runtime: HermesRuntime,
    should_randomize_identifiers: bool,
    chain_store_dir: &str,
    transfer_denom_prefix: String,
    genesis_modifier: impl Fn(&mut JsonValue) -> Result<(), Error> + Send + Sync + 'static,
    comet_config_modifier: impl Fn(&mut TomlValue) -> Result<(), Error> + Send + Sync + 'static,
) -> CosmosBootstrap {
    let cosmos_builder = Arc::new(CosmosBuilder::new_with_default(runtime.clone()));

    CosmosBootstrap {
        runtime,
        cosmos_builder,
        should_randomize_identifiers,
        chain_store_dir: chain_store_dir.into(),
        chain_command_path: "gaiad".into(),
        account_prefix: "cosmos".into(),
        staking_denom_prefix: "stake".into(),
        transfer_denom_prefix,
        genesis_config_modifier: Box::new(genesis_modifier),
        comet_config_modifier: Box::new(comet_config_modifier),
        dynamic_gas: Some(DynamicGasConfig::default()),
    }
}

pub fn init_preset_bootstraps(
    runtime: &HermesRuntime,
) -> Result<(CosmosBootstrap, CosmosBootstrap), Error> {
    let test_preset = env::var("TEST_PRESET")
        .unwrap_or_else(|_| "CosmosToCosmos".to_string())
        .parse::<TestPreset>()?;

    match test_preset {
        TestPreset::CosmosToCosmos => {
            let bootstrap_chain_0 = build_gaia_bootstrap(
                runtime.clone(),
                true,
                "./test-data",
                "coin".into(),
                |_| Ok(()),
                |_| Ok(()),
            );

            let bootstrap_chain_1 = build_gaia_bootstrap(
                runtime.clone(),
                true,
                "./test-data",
                "coin".into(),
                |_| Ok(()),
                |_| Ok(()),
            );

            Ok((bootstrap_chain_0, bootstrap_chain_1))
        }
    }
}
