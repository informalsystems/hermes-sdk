use core::time::Duration;
use std::sync::Arc;

use eyre::Error;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;
use hermes_relayer_runtime::types::runtime::HermesRuntime;
use hermes_test_components::setup::traits::driver::CanBuildTestDriver;
use ibc_relayer::chain::client::ClientSettings;
use ibc_relayer::chain::cosmos::client::Settings;
use ibc_relayer_types::core::ics02_client::trust_threshold::TrustThreshold;
use ibc_relayer_types::core::ics24_host::identifier::PortId;
use tokio::runtime::Runtime;
use tokio::test;

use crate::contexts::binary_channel::setup::CosmosBinaryChannelSetup;
use crate::contexts::bootstrap::CosmosBootstrap;

#[test(flavor = "multi_thread")]
async fn test_setup_cosmos_chain() -> Result<(), Error> {
    let _ = stable_eyre::install();

    let tokio_runtime = Arc::new(Runtime::new()?);
    let runtime = HermesRuntime::new(tokio_runtime.clone());

    let builder = CosmosBuilder::new_with_default(runtime.clone());

    let bootstrap = CosmosBootstrap {
        runtime,
        builder,
        should_randomize_identifiers: true,
        test_dir: "./test-data".into(),
        chain_command_path: "gaiad".into(),
        account_prefix: "cosmos".into(),
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

    setup.build_driver().await?;

    Ok(())
}
