#![recursion_limit = "256"]

use core::time::Duration;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use futures::StreamExt;
use hermes_celestia_integration_tests::contexts::bootstrap::CelestiaBootstrap;
use hermes_celestia_test_components::bootstrap::traits::bootstrap_bridge::CanBootstrapBridge;
use hermes_cosmos_integration_tests::contexts::bootstrap::CosmosBootstrap;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_cosmos_relayer::types::error::Error;
use hermes_relayer_components::chain::traits::queries::client_state::CanQueryClientStateWithLatestHeight;
use hermes_relayer_components::chain::traits::queries::consensus_state_height::CanQueryConsensusStateHeights;
use hermes_relayer_components::relay::traits::client_creator::CanCreateClient;
use hermes_relayer_components::relay::traits::target::DestinationTarget;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_sovereign_chain_components::sovereign::traits::chain::rollup::HasRollup;
use hermes_sovereign_integration_tests::contexts::bootstrap::SovereignBootstrap;
use hermes_sovereign_relayer::contexts::cosmos_to_sovereign_relay::CosmosToSovereignRelay;
use hermes_sovereign_relayer::contexts::sovereign_chain::SovereignChain;
use hermes_sovereign_relayer::contexts::sovereign_rollup::SovereignRollup;
use hermes_sovereign_test_components::bootstrap::traits::bootstrap_rollup::CanBootstrapRollup;
use hermes_test_components::bootstrap::traits::chain::CanBootstrapChain;
use hermes_test_components::chain_driver::traits::types::chain::HasChain;
use ibc_relayer::chain::client::ClientSettings;
use ibc_relayer::chain::cosmos::client::Settings;
use ibc_relayer_types::core::ics02_client::trust_threshold::TrustThreshold;
use jsonrpsee::core::client::{Subscription, SubscriptionClientT};
use jsonrpsee::core::params::ArrayParams;
use tokio::runtime::Builder;
use tokio::time::sleep;

#[test]
fn test_cosmos_to_sovereign() -> Result<(), Error> {
    let _ = stable_eyre::install();

    let tokio_runtime = Arc::new(Builder::new_multi_thread().enable_all().build()?);

    let runtime = HermesRuntime::new(tokio_runtime.clone());

    let builder = Arc::new(CosmosBuilder::new_with_default(runtime.clone()));

    let store_postfix = format!(
        "{}-{}",
        SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis(),
        rand::random::<u64>()
    );

    let store_dir = std::env::current_dir()?.join(format!("test-data/{store_postfix}"));

    let cosmos_bootstrap = Arc::new(CosmosBootstrap {
        runtime: runtime.clone(),
        builder: builder.clone(),
        should_randomize_identifiers: true,
        chain_store_dir: store_dir.join("chains"),
        chain_command_path: "simd".into(),
        account_prefix: "cosmos".into(),
        staking_denom: "stake".into(),
        transfer_denom: "coin".into(),
        genesis_config_modifier: Box::new(|_| Ok(())),
        comet_config_modifier: Box::new(|_| Ok(())),
    });

    let celestia_bootstrap = CelestiaBootstrap {
        runtime: runtime.clone(),
        builder: builder.clone(),
        chain_store_dir: store_dir.join("chains"),
        bridge_store_dir: store_dir.join("bridges"),
    };

    let sovereign_bootstrap = SovereignBootstrap {
        runtime: runtime.clone(),
        rollup_store_dir: store_dir.join("rollups"),
        rollup_command_path: "node".into(),
        account_prefix: "sov".into(),
    };

    tokio_runtime.block_on(async move {
        let cosmos_chain_driver = cosmos_bootstrap.bootstrap_chain("cosmos").await?;

        let celestia_chain_driver = celestia_bootstrap.bootstrap_chain("private").await?;

        let bridge_driver = celestia_bootstrap
            .bootstrap_bridge(&celestia_chain_driver)
            .await?;

        let rollup_driver = sovereign_bootstrap
            .bootstrap_rollup(&celestia_chain_driver, &bridge_driver, "test-rollup")
            .await?;

        let cosmos_chain = cosmos_chain_driver.chain();
        let rollup = rollup_driver.rollup();

        let sovereign_chain = SovereignChain {
            runtime: runtime.clone(),
            data_chain: celestia_chain_driver.chain().clone(),
            rollup: rollup.clone(),
        };

        {
            let subscription: Subscription<u64> = rollup
                .subscription_client
                .subscribe(
                    "ledger_subscribeSlots",
                    ArrayParams::new(),
                    "ledger_unsubscribeSlots",
                )
                .await?;

            runtime.runtime.spawn(async move {
                subscription
                    .for_each(|value| async move {
                        println!("slot subscription yields: {:?}", value);
                    })
                    .await;
            });
        }

        {
            let create_client_settings = ClientSettings::Tendermint(Settings {
                max_clock_drift: Duration::from_secs(40),
                trusting_period: None,
                trust_threshold: TrustThreshold::ONE_THIRD,
            });

            sleep(Duration::from_secs(1)).await;

            let client_id = CosmosToSovereignRelay::create_client(
                DestinationTarget,
                &sovereign_chain,
                cosmos_chain,
                &create_client_settings,
            )
            .await?;

            println!("client ID of Cosmos on Sovereign: {:?}", client_id);

            let client_state = <SovereignRollup as CanQueryClientStateWithLatestHeight<
                CosmosChain,
            >>::query_client_state_with_latest_height(
                rollup, &client_id
            )
            .await?;

            println!("client state: {:?}", client_state);

            let consensus_state_heights = <SovereignRollup as CanQueryConsensusStateHeights<
                CosmosChain,
            >>::query_consensus_state_heights(
                rollup, &client_id
            )
            .await?;

            println!("consensus state heights: {:?}", consensus_state_heights);
        }

        <Result<(), Error>>::Ok(())
    })?;

    Ok(())
}
