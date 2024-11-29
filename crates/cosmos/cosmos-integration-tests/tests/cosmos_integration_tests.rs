#![recursion_limit = "256"]

use std::sync::Arc;

use hermes_cosmos_integration_tests::contexts::binary_channel::setup::CosmosBinaryChannelSetup;
use hermes_cosmos_integration_tests::init::{init_preset_bootstraps, init_test_runtime};
use hermes_error::types::Error;
use hermes_ibc_test_suite::tests::transfer::TestIbcTransfer;
use hermes_test_components::setup::traits::run_test::CanRunTest;

use ibc_relayer_types::core::ics24_host::identifier::PortId;

#[test]
fn cosmos_integration_tests() -> Result<(), Error> {
    let runtime = init_test_runtime();

    let (bootstrap_chain_0, bootstrap_chain_1) = init_preset_bootstraps(&runtime)?;

    let setup = CosmosBinaryChannelSetup {
        bootstrap_a: Arc::new(bootstrap_chain_0),
        bootstrap_b: Arc::new(bootstrap_chain_1),
        create_client_settings: Default::default(),
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
