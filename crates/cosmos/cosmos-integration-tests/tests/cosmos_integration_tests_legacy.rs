#![recursion_limit = "256"]

use core::time::Duration;
use std::sync::Arc;

use hermes_cosmos_chain_components::traits::eip::eip_type::EipQueryType;
use hermes_cosmos_integration_tests::contexts::binary_channel::setup_legacy::LegacyCosmosBinaryChannelSetup;
use hermes_cosmos_integration_tests::contexts::bootstrap_legacy::LegacyCosmosBootstrap;
use hermes_cosmos_integration_tests::init::init_test_runtime;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
use hermes_cosmos_test_components::types::dynamic_gas_config::DynamicGasConfig;
use hermes_error::types::Error;
use hermes_ibc_test_suite::tests::transfer::TestIbcTransfer;
use hermes_test_components::setup::traits::run_test::CanRunTest;
use ibc_relayer::chain::cosmos::client::Settings;
use ibc_relayer_types::core::ics02_client::trust_threshold::TrustThreshold;
use ibc_relayer_types::core::ics24_host::identifier::PortId;

#[test]
fn cosmos_integration_tests() -> Result<(), Error> {
    let maybe_dynamic_gas_fee_config = std::env::var("DYNAMIC_GAS_MULTIPLIER")
        .ok()
        .and_then(|dynamic_gas_multiplier| dynamic_gas_multiplier.parse::<f64>().ok())
        .map(|f64_dynamic_gas_multiplier| DynamicGasConfig {
            multiplier: f64_dynamic_gas_multiplier,
            max: 2.0,
        });

    let runtime = init_test_runtime();

    // Note: This test only works with Gaia v14 or older. Hence we get the older version of
    // gaiad from the environment variable, if applicable.
    let gaia_bin = std::env::var("LEGACY_GAIA_BIN").unwrap_or("gaiad".into());

    let builder = Arc::new(CosmosBuilder::new_with_default(runtime.clone()));

    // TODO: load parameters from environment variables
    let bootstrap = Arc::new(LegacyCosmosBootstrap {
        runtime: runtime.clone(),
        cosmos_builder: builder,
        should_randomize_identifiers: true,
        chain_store_dir: "./test-data".into(),
        chain_command_path: gaia_bin.into(),
        account_prefix: "cosmos".into(),
        compat_mode: None,
        staking_denom_prefix: "stake".into(),
        transfer_denom_prefix: "coin".into(),
        genesis_config_modifier: Box::new(|_| Ok(())),
        comet_config_modifier: Box::new(|_| Ok(())),
        dynamic_gas: maybe_dynamic_gas_fee_config,
        eip_query_type: EipQueryType::FeeMarket,
    });

    let create_client_settings = Settings {
        max_clock_drift: Duration::from_secs(40),
        trusting_period: None,
        trust_threshold: TrustThreshold::ONE_THIRD,
    };

    let setup = LegacyCosmosBinaryChannelSetup {
        bootstrap_a: bootstrap.clone(),
        bootstrap_b: bootstrap,
        create_client_settings,
        init_connection_options: Default::default(),
        init_channel_options: Default::default(),
        port_id: PortId::transfer(),
    };

    // TODO: Use a test suite entry point for running multiple tests
    runtime.runtime.clone().block_on(async move {
        setup.run_test(&TestIbcTransfer).await?;

        <Result<(), Error>>::Ok(())
    })?;

    Ok(())
}
