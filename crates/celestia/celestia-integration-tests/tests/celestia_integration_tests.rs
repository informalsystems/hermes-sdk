#![recursion_limit = "256"]

use std::sync::Arc;

use hermes_cosmos_chain_components::types::DynamicGasConfig;
use hermes_cosmos_integration_tests::contexts::binary_channel::setup::CosmosBinaryChannelSetup;
use hermes_cosmos_integration_tests::contexts::bootstrap::{
    CosmosBootstrap, CosmosBootstrapFields,
};
use hermes_cosmos_integration_tests::init::init_test_runtime;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
use hermes_error::types::Error;
use hermes_ibc_test_suite::tests::transfer::TestIbcTransfer;
use hermes_test_components::setup::traits::CanRunTest;

// FIXME: Celestia currently can only be bootstrapped using CosmosBootstrap.
// We need to refactor `CosmosBinaryChannelSetup` to make it support generic
// bootstrap contexts.
#[test]
#[ignore]
fn celestia_integration_tests() -> Result<(), Error> {
    let runtime = init_test_runtime();

    let builder = CosmosBuilder::new_with_default(runtime.clone());

    let celestia_bootstrap = CosmosBootstrap {
        fields: Arc::new(CosmosBootstrapFields {
            runtime: runtime.clone(),
            cosmos_builder: builder.clone(),
            should_randomize_identifiers: true,
            chain_store_dir: "./test-data/chains".into(),
            chain_command_path: "celestia-appd".into(),
            account_prefix: "celestia".into(),
            staking_denom_prefix: "utia".into(),
            transfer_denom_prefix: "coin".into(),
            genesis_config_modifier: Box::new(|_| Ok(())),
            comet_config_modifier: Box::new(|_| Ok(())),
            dynamic_gas: Some(DynamicGasConfig::default()),
        }),
    };

    let cosmos_bootstrap = CosmosBootstrap {
        fields: Arc::new(CosmosBootstrapFields {
            runtime: runtime.clone(),
            cosmos_builder: builder.clone(),
            should_randomize_identifiers: true,
            chain_store_dir: "./test-data/chains".into(),
            chain_command_path: "gaiad".into(),
            account_prefix: "cosmos".into(),
            staking_denom_prefix: "stake".into(),
            transfer_denom_prefix: "coin".into(),
            genesis_config_modifier: Box::new(|_| Ok(())),
            comet_config_modifier: Box::new(|_| Ok(())),
            dynamic_gas: Some(DynamicGasConfig::default()),
        }),
    };

    let setup =
        CosmosBinaryChannelSetup::new_with_defaults(celestia_bootstrap, cosmos_bootstrap, builder);

    runtime.runtime.clone().block_on(async move {
        setup.run_test(&TestIbcTransfer::default()).await?;

        <Result<(), Error>>::Ok(())
    })?;

    Ok(())
}
