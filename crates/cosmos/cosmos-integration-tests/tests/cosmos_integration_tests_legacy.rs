#![recursion_limit = "256"]
use std::sync::Arc;

use hermes_cosmos_chain_components::types::config::gas::dynamic_gas_config::DynamicGasConfig;
use hermes_cosmos_integration_tests::contexts::binary_channel::setup::CosmosBinaryChannelSetup;
use hermes_cosmos_integration_tests::contexts::bootstrap_legacy::{
    LegacyCosmosBootstrap, LegacyCosmosBootstrapFields,
};
use hermes_cosmos_integration_tests::init::init_test_runtime;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
use hermes_error::types::Error;
use hermes_ibc_test_suite::tests::transfer::TestIbcTransfer;
use hermes_test_components::setup::traits::run_test::CanRunTest;
use ibc_relayer_types::core::ics24_host::identifier::PortId;

#[test]
fn cosmos_integration_tests_legacy() -> Result<(), Error> {
    let runtime = init_test_runtime();

    // Note: This test only works with Gaia v14 or older. Hence we get the older version of
    // gaiad from the environment variable, if applicable.
    let dynamic_gas_config = Some(DynamicGasConfig::new(1.1, 1.6, "osmosis", "stake"));

    let builder = CosmosBuilder::new_with_default(runtime.clone());

    // TODO: load parameters from environment variables
    let bootstrap = LegacyCosmosBootstrap {
        fields: Arc::new(LegacyCosmosBootstrapFields {
            runtime: runtime.clone(),
            cosmos_builder: builder.clone(),
            should_randomize_identifiers: true,
            chain_store_dir: "./test-data".into(),
            chain_command_path: "osmosisd".into(),
            account_prefix: "osmo".into(),
            compat_mode: None,
            staking_denom_prefix: "stake".into(),
            transfer_denom_prefix: "coin".into(),
            genesis_config_modifier: Box::new(|_| Ok(())),
            comet_config_modifier: Box::new(|_| Ok(())),
            dynamic_gas: dynamic_gas_config,
        }),
    };

    let setup = CosmosBinaryChannelSetup {
        builder,
        bootstrap_a: bootstrap.clone(),
        bootstrap_b: bootstrap,
        create_client_payload_options: Default::default(),
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
