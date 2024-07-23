#![recursion_limit = "256"]

use core::time::Duration;
use std::sync::Arc;

use hermes_cosmos_integration_tests::contexts::binary_channel::setup::CosmosBinaryChannelSetup;
use hermes_cosmos_integration_tests::contexts::bootstrap_legacy::LegacyCosmosBootstrap;
use hermes_cosmos_integration_tests::init::init_test_runtime;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
use hermes_error::types::Error;
use hermes_ibc_test_suite::tests::transfer::TestIbcTransfer;
use hermes_test_components::setup::traits::run_test::CanRunTest;
use ibc_relayer::chain::cosmos::client::Settings;
use ibc_relayer::config::compat_mode::CompatMode;
use ibc_relayer_types::core::ics02_client::trust_threshold::TrustThreshold;
use ibc_relayer_types::core::ics24_host::identifier::PortId;

#[test]
fn celestia_integration_tests() -> Result<(), Error> {
    let runtime = init_test_runtime();

    let builder = Arc::new(CosmosBuilder::new_with_default(runtime.clone()));

    let celestia_bootstrap = Arc::new(LegacyCosmosBootstrap {
        runtime: runtime.clone(),
        builder: builder.clone(),
        should_randomize_identifiers: true,
        chain_store_dir: "./test-data/chains".into(),
        chain_command_path: "celestia-appd".into(),
        account_prefix: "celestia".into(),
        compat_mode: Some(CompatMode::V0_34),
        staking_denom: "utia".into(),
        transfer_denom: "coin".into(),
        genesis_config_modifier: Box::new(|_| Ok(())),
        comet_config_modifier: Box::new(|_| Ok(())),
    });

    let cosmos_bootstrap = Arc::new(LegacyCosmosBootstrap {
        runtime: runtime.clone(),
        builder,
        should_randomize_identifiers: true,
        chain_store_dir: "./test-data/chains".into(),
        chain_command_path: "gaiad".into(),
        account_prefix: "cosmos".into(),
        compat_mode: None,
        staking_denom: "stake".into(),
        transfer_denom: "coin".into(),
        genesis_config_modifier: Box::new(|_| Ok(())),
        comet_config_modifier: Box::new(|_| Ok(())),
    });

    let create_client_settings = Settings {
        max_clock_drift: Duration::from_secs(40),
        trusting_period: Some(Duration::from_secs(60 * 60)),
        trust_threshold: TrustThreshold::ONE_THIRD,
    };

    let setup = CosmosBinaryChannelSetup {
        bootstrap_a: celestia_bootstrap,
        bootstrap_b: cosmos_bootstrap,
        create_client_settings,
        init_connection_options: Default::default(),
        init_channel_options: Default::default(),
        port_id: PortId::transfer(),
    };

    runtime.runtime.clone().block_on(async move {
        setup.run_test(&TestIbcTransfer).await?;

        <Result<(), Error>>::Ok(())
    })?;

    Ok(())
}
