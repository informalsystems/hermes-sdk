use core::time::Duration;
use std::sync::Arc;

use eyre::Error;
use hermes_cosmos_client_components::types::channel::CosmosInitChannelOptions;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;
use hermes_cosmos_relayer::contexts::relay::CosmosRelay;
use hermes_relayer_components::build::traits::components::relay_from_chains_builder::CanBuildRelayFromChains;
use hermes_relayer_components::build::traits::target::relay::RelayAToBTarget;
use hermes_relayer_components::relay::impls::channel::bootstrap::CanBootstrapChannel;
use hermes_relayer_components::relay::impls::connection::bootstrap::CanBootstrapConnection;
use hermes_relayer_components::relay::traits::components::client_creator::CanCreateClient;
use hermes_relayer_components::relay::traits::target::{DestinationTarget, SourceTarget};
use hermes_relayer_runtime::types::runtime::HermesRuntime;
use hermes_test_components::bootstrap::traits::chain::CanBootstrapChain;
use ibc_relayer::chain::client::ClientSettings;
use ibc_relayer::chain::cosmos::client::Settings;
use ibc_relayer_types::core::ics02_client::trust_threshold::TrustThreshold;
use ibc_relayer_types::core::ics24_host::identifier::PortId;
use tokio::runtime::Runtime;
use tokio::test;
use tokio::time::sleep;

use crate::contexts::bootstrap::CosmosBootstrap;

#[test(flavor = "multi_thread")]
async fn test_bootstrap_cosmos_chain() -> Result<(), Error> {
    stable_eyre::install()?;

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

    let chain_a = bootstrap.bootstrap_chain("cosmos-testnet-1").await?;

    let chain_b = bootstrap.bootstrap_chain("cosmos-testnet-2").await?;

    sleep(Duration::from_secs(2)).await;

    let client_settings = ClientSettings::Tendermint(Settings {
        max_clock_drift: Duration::from_secs(40),
        trusting_period: None,
        trust_threshold: TrustThreshold::ONE_THIRD,
    });

    println!("client settings: {:?}", client_settings);

    let client_id_a = CosmosRelay::create_client(
        SourceTarget,
        &chain_a.base_chain,
        &chain_b.base_chain,
        &client_settings,
    )
    .await?;

    let client_id_b = CosmosRelay::create_client(
        DestinationTarget,
        &chain_b.base_chain,
        &chain_a.base_chain,
        &client_settings,
    )
    .await?;

    let relay = bootstrap
        .builder
        .build_relay_from_chains(
            RelayAToBTarget,
            &client_id_a,
            &client_id_b,
            chain_a.base_chain.clone(),
            chain_b.base_chain.clone(),
        )
        .await?;

    let (connection_id_a, _connection_id_b) =
        relay.bootstrap_connection(&Default::default()).await?;

    let init_channel_option = CosmosInitChannelOptions {
        ordering: Default::default(),
        connection_hops: vec![connection_id_a],
        channel_version: Default::default(),
    };

    relay
        .bootstrap_channel(
            &PortId::transfer(),
            &PortId::transfer(),
            &init_channel_option,
        )
        .await?;

    Ok(())
}
