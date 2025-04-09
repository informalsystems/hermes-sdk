use alloc::sync::Arc;
use std::env;
use std::str::FromStr;

use eyre::Report;
use hermes_cosmos_chain_components::types::config::gas::dynamic_gas_config::DynamicGasConfig;
use hermes_cosmos_chain_components::types::messages::packet::packet_filter::PacketFilterConfig;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
use hermes_error::types::Error;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_test_components::setup::traits::driver::CanBuildTestDriver;
use hermes_tracing_logging_components::subscriber::init_tracing_subscriber;
use serde_json::Value as JsonValue;
use tokio::runtime::Builder;
use toml::Value as TomlValue;
use tracing::info;

use crate::contexts::binary_channel::setup::CosmosBinaryChannelSetup;
use crate::contexts::binary_channel::test_driver::CosmosBinaryChannelTestDriver;
use crate::contexts::bootstrap::{CosmosBootstrap, CosmosBootstrapFields};
use crate::contexts::bootstrap_legacy::{LegacyCosmosBootstrap, LegacyCosmosBootstrapFields};

pub enum TestPreset {
    GaiaToGaia,
    OsmosisToOsmosis,
    OsmosisToGaia,
}

impl FromStr for TestPreset {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "gaiatogaia" => Ok(TestPreset::GaiaToGaia),
            "osmosistoosmosis" => Ok(TestPreset::OsmosisToOsmosis),
            "osmosistogaia" => Ok(TestPreset::OsmosisToGaia),
            _ => Err(Report::msg("unknown test preset: `{s}`")),
        }
    }
}

pub fn init_test_runtime() -> HermesRuntime {
    let _ = stable_eyre::install();

    init_tracing_subscriber();

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
    packet_filter: PacketFilterConfig,
) -> LegacyCosmosBootstrap {
    let dynamic_gas_config = Some(DynamicGasConfig::new(1.1, 1.6, "osmosis", "stake"));
    let cosmos_builder = CosmosBuilder::new(
        Default::default(),
        runtime.clone(),
        Default::default(),
        packet_filter,
        Default::default(),
        Default::default(),
    );

    LegacyCosmosBootstrap {
        fields: Arc::new(LegacyCosmosBootstrapFields {
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
            dynamic_gas: dynamic_gas_config,
        }),
    }
}

pub fn build_gaia_bootstrap(
    runtime: HermesRuntime,
    should_randomize_identifiers: bool,
    chain_store_dir: &str,
    transfer_denom_prefix: String,
    genesis_modifier: impl Fn(&mut JsonValue) -> Result<(), Error> + Send + Sync + 'static,
    comet_config_modifier: impl Fn(&mut TomlValue) -> Result<(), Error> + Send + Sync + 'static,
    packet_filter: PacketFilterConfig,
) -> CosmosBootstrap {
    let dynamic_gas_config = Some(DynamicGasConfig::default());
    let cosmos_builder = CosmosBuilder::new(
        Default::default(),
        runtime.clone(),
        Default::default(),
        packet_filter,
        Default::default(),
        Default::default(),
    );

    CosmosBootstrap {
        fields: Arc::new(CosmosBootstrapFields {
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
            dynamic_gas: dynamic_gas_config,
        }),
    }
}

async fn setup_gaia_to_gaia(
    runtime: &HermesRuntime,
    builder: CosmosBuilder,
    packet_filter: PacketFilterConfig,
) -> Result<CosmosBinaryChannelTestDriver, Error> {
    let bootstrap_chain_0 = build_gaia_bootstrap(
        runtime.clone(),
        true,
        "./test-data",
        "coin".into(),
        |_| Ok(()),
        |_| Ok(()),
        packet_filter.clone(),
    );

    let bootstrap_chain_1 = build_gaia_bootstrap(
        runtime.clone(),
        true,
        "./test-data",
        "coin".into(),
        |_| Ok(()),
        |_| Ok(()),
        packet_filter,
    );

    let setup =
        CosmosBinaryChannelSetup::new_with_defaults(bootstrap_chain_0, bootstrap_chain_1, builder);

    setup.build_driver().await
}

async fn setup_osmosis_to_osmosis(
    runtime: &HermesRuntime,
    builder: CosmosBuilder,
    packet_filter: PacketFilterConfig,
) -> Result<CosmosBinaryChannelTestDriver, Error> {
    let bootstrap_chain_0 = build_osmosis_bootstrap(
        runtime.clone(),
        true,
        "./test-data",
        "coin".into(),
        |_| Ok(()),
        |_| Ok(()),
        packet_filter.clone(),
    );

    let bootstrap_chain_1 = build_osmosis_bootstrap(
        runtime.clone(),
        true,
        "./test-data",
        "coin".into(),
        |_| Ok(()),
        |_| Ok(()),
        packet_filter,
    );

    let setup =
        CosmosBinaryChannelSetup::new_with_defaults(bootstrap_chain_0, bootstrap_chain_1, builder);

    setup.build_driver().await
}

async fn setup_osmosis_to_gaia(
    runtime: &HermesRuntime,
    builder: CosmosBuilder,
    packet_filter: PacketFilterConfig,
) -> Result<CosmosBinaryChannelTestDriver, Error> {
    let bootstrap_chain_0 = build_osmosis_bootstrap(
        runtime.clone(),
        true,
        "./test-data",
        "coin".into(),
        |_| Ok(()),
        |_| Ok(()),
        packet_filter.clone(),
    );

    let bootstrap_chain_1 = build_gaia_bootstrap(
        runtime.clone(),
        true,
        "./test-data",
        "coin".into(),
        |_| Ok(()),
        |_| Ok(()),
        packet_filter,
    );

    let setup =
        CosmosBinaryChannelSetup::new_with_defaults(bootstrap_chain_0, bootstrap_chain_1, builder);

    setup.build_driver().await
}

pub async fn init_preset_bootstraps(
    runtime: &HermesRuntime,
    packet_filter: PacketFilterConfig,
) -> Result<CosmosBinaryChannelTestDriver, Error> {
    let test_preset = env::var("TEST_PRESET")
        .unwrap_or_else(|_| "GaiaToGaia".to_string())
        .parse::<TestPreset>()?;

    let builder = CosmosBuilder::new(
        Default::default(),
        runtime.clone(),
        Default::default(),
        packet_filter.clone(),
        Default::default(),
        Default::default(),
    );

    match test_preset {
        TestPreset::GaiaToGaia => setup_gaia_to_gaia(runtime, builder, packet_filter).await,
        TestPreset::OsmosisToOsmosis => {
            setup_osmosis_to_osmosis(runtime, builder, packet_filter).await
        }
        TestPreset::OsmosisToGaia => setup_osmosis_to_gaia(runtime, builder, packet_filter).await,
    }
}
