#![recursion_limit = "256"]

use core::time::Duration;
use std::sync::Arc;

use hermes_cosmos_chain_components::types::payloads::client::CosmosCreateClientOptions;
use hermes_cosmos_integration_tests::contexts::binary_channel::setup_legacy::LegacyCosmosBinaryChannelSetup;
use hermes_cosmos_integration_tests::contexts::bootstrap_legacy::LegacyCosmosBootstrap;
use hermes_cosmos_integration_tests::init::init_test_runtime;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
use hermes_error::types::Error;
use hermes_ibc_test_suite::tests::transfer::TestIbcTransfer;
use hermes_test_components::setup::traits::run_test::CanRunTest;
use ibc_proto::ibc::lightclients::tendermint::v1::Fraction;
use ibc_relayer_types::core::ics24_host::identifier::PortId;

#[test]
fn cosmos_integration_tests_legacy() -> Result<(), Error> {
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
    });

    let create_client_settings = CosmosCreateClientOptions {
        max_clock_drift: Duration::from_secs(40),
        trust_threshold: Fraction {
            numerator: 1,
            denominator: 2,
        },
        ..Default::default()
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
