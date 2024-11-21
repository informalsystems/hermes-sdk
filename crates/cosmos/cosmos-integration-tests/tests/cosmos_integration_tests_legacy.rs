#![recursion_limit = "256"]

use core::time::Duration;
use std::sync::Arc;

use hermes_cosmos_integration_tests::contexts::binary_channel::setup_legacy::LegacyCosmosBinaryChannelSetup;
use hermes_cosmos_integration_tests::init::{init_bootstrap_legacy, init_test_runtime};
use hermes_error::types::Error;
use hermes_ibc_test_suite::tests::transfer::TestIbcTransfer;
use hermes_test_components::setup::traits::run_test::CanRunTest;
use ibc_relayer::chain::cosmos::client::Settings;
use ibc_relayer_types::core::ics02_client::trust_threshold::TrustThreshold;
use ibc_relayer_types::core::ics24_host::identifier::PortId;

#[test]
fn cosmos_integration_tests() -> Result<(), Error> {
    let runtime = init_test_runtime();

    // Use this dynamic gas configuration if running test with Osmosis
    //let dynamic_gas = Some(DynamicGasConfig::new(1.1, 1.6, "osmosis", "stake"));
    let dynamic_gas = None;

    let bootstrap_0 = Arc::new(init_bootstrap_legacy(
        0,
        runtime.clone(),
        true,
        "./test-data",
        "coin".into(),
        |_| Ok(()),
        |_| Ok(()),
        dynamic_gas.clone(),
    ));

    let bootstrap_1 = Arc::new(init_bootstrap_legacy(
        1,
        runtime.clone(),
        true,
        "./test-data",
        "coin".into(),
        |_| Ok(()),
        |_| Ok(()),
        dynamic_gas,
    ));

    let create_client_settings = Settings {
        max_clock_drift: Duration::from_secs(40),
        trusting_period: None,
        trust_threshold: TrustThreshold::ONE_THIRD,
    };

    let setup = LegacyCosmosBinaryChannelSetup {
        bootstrap_a: bootstrap_0,
        bootstrap_b: bootstrap_1,
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
