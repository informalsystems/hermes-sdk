use core::time::Duration;
use std::sync::Arc;

use eyre::Error;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;
use hermes_cosmos_test_components::chain_driver::types::denom::Denom;
use hermes_ibc_test_suite::tests::transfer::TestIbcTransfer;
use hermes_relayer_runtime::types::runtime::HermesRuntime;
use hermes_test_components::setup::traits::run_test::CanRunTest;
use ibc_relayer::chain::client::ClientSettings;
use ibc_relayer::chain::cosmos::client::Settings;
use ibc_relayer_types::core::ics02_client::trust_threshold::TrustThreshold;
use ibc_relayer_types::core::ics24_host::identifier::PortId;
use tokio::runtime::Builder;

use crate::contexts::binary_channel::setup::CosmosBinaryChannelSetup;
use crate::contexts::bootstrap::CosmosBootstrap;

#[test]
fn cosmos_integration_tests() -> Result<(), Error> {
    let _ = stable_eyre::install();

    let tokio_runtime = Arc::new(Builder::new_multi_thread().enable_all().build()?);

    let runtime = HermesRuntime::new(tokio_runtime.clone());

    let builder = CosmosBuilder::new_with_default(runtime.clone());

    // TODO: load parameters from environment variables
    let bootstrap = CosmosBootstrap {
        runtime,
        builder,
        should_randomize_identifiers: true,
        test_dir: "./test-data".into(),
        chain_command_path: "gaiad".into(),
        account_prefix: "cosmos".into(),
        compat_mode: None,
        staking_denom: Denom::base("stake"),
        transfer_denom: Denom::base("coin"),
        genesis_config_modifier: Box::new(|_| Ok(())),
        comet_config_modifier: Box::new(|_| Ok(())),
    };

    let create_client_settings = ClientSettings::Tendermint(Settings {
        max_clock_drift: Duration::from_secs(40),
        trusting_period: None,
        trust_threshold: TrustThreshold::ONE_THIRD,
    });

    let setup = CosmosBinaryChannelSetup {
        bootstrap,
        create_client_settings,
        init_connection_options: Default::default(),
        init_channel_options: Default::default(),
        port_id: PortId::transfer(),
    };

    // TODO: Use a test suite entry point for running multiple tests
    tokio_runtime.block_on(async move {
        setup.run_test(&TestIbcTransfer).await?;

        <Result<(), Error>>::Ok(())
    })?;

    Ok(())
}
