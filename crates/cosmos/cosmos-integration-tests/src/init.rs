use alloc::sync::Arc;
use std::env;
use std::str::FromStr;

use eyre::Report;
use hermes_core::test_components::setup::traits::CanBuildTestDriver;
use hermes_cosmos_core::chain_components::types::{DynamicGasConfig, PacketFilterConfig};
use hermes_cosmos_core::tracing_logging_components::subscriber::init_tracing_subscriber;
use hermes_cosmos_relayer::contexts::CosmosBuilder;
use hermes_error::types::Error;
use hermes_runtime::types::runtime::HermesRuntime;
use serde_json::Value as JsonValue;
use tokio::runtime::Builder;
use toml::Value as TomlValue;
use tracing::info;

use crate::contexts::{
    CosmosBinaryChannelSetup, CosmosBinaryChannelTestDriver, CosmosBootstrap,
    CosmosBootstrapFields, LegacyCosmosBootstrap, LegacyCosmosBootstrapFields,
};

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
        |config| {
            let gov_params = config
                .get_mut("app_state")
                .and_then(|app_state| app_state.get_mut("gov"))
                .and_then(|gov| gov.get_mut("params"))
                .and_then(|gov_params| gov_params.as_object_mut())
                .unwrap();

            gov_params.insert(
                "max_deposit_period".to_owned(),
                JsonValue::String("6s".to_owned()),
            );

            if gov_params.contains_key("expedited_voting_period") {
                gov_params.insert(
                    "expedited_voting_period".to_owned(),
                    JsonValue::String("5s".to_owned()),
                );
            }

            let voting_period = config
                .get_mut("app_state")
                .and_then(|app_state| app_state.get_mut("gov"))
                .and_then(|gov| gov.get_mut("params"))
                .and_then(|voting_params| voting_params.as_object_mut())
                .unwrap();

            voting_period.insert(
                "voting_period".to_owned(),
                serde_json::Value::String("10s".to_owned()),
            );
            Ok(())
        },
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
