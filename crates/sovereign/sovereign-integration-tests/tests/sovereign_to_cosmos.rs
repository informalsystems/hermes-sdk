#![recursion_limit = "256"]

use core::time::Duration;
use std::collections::BTreeSet;
use std::env::var;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use futures::lock::Mutex;
use hermes_celestia_integration_tests::contexts::bootstrap::CelestiaBootstrap;
use hermes_celestia_test_components::bootstrap::traits::bootstrap_bridge::CanBootstrapBridge;
use hermes_cosmos_chain_components::types::channel::CosmosInitChannelOptions;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;
use hermes_cosmos_relayer::types::error::Error;
use hermes_cosmos_wasm_relayer::context::cosmos_bootstrap::CosmosWithWasmClientBootstrap;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainId;
use hermes_relayer_components::relay::impls::channel::bootstrap::CanBootstrapChannel;
use hermes_relayer_components::relay::impls::connection::bootstrap::CanBootstrapConnection;
use hermes_relayer_components::relay::traits::client_creator::CanCreateClient;
use hermes_relayer_components::relay::traits::target::{DestinationTarget, SourceTarget};
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_sovereign_chain_components::sovereign::traits::chain::rollup::HasRollup;
use hermes_sovereign_chain_components::sovereign::types::payloads::client::SovereignCreateClientOptions;
use hermes_sovereign_integration_tests::contexts::sovereign_bootstrap::SovereignBootstrap;
use hermes_sovereign_relayer::contexts::sovereign_chain::SovereignChain;
use hermes_sovereign_relayer::contexts::sovereign_to_cosmos_relay::SovereignToCosmosRelay;
use hermes_sovereign_test_components::bootstrap::traits::bootstrap_rollup::CanBootstrapRollup;
use hermes_test_components::bootstrap::traits::chain::CanBootstrapChain;
use hermes_test_components::chain::traits::queries::balance::CanQueryBalance;
use hermes_test_components::chain_driver::traits::types::chain::HasChain;
use ibc::core::client::types::Height;
use ibc_relayer::chain::client::ClientSettings;
use ibc_relayer::chain::cosmos::client::Settings;
use ibc_relayer_types::core::ics02_client::trust_threshold::TrustThreshold;
use ibc_relayer_types::core::ics24_host::identifier::PortId;
use sha2::{Digest, Sha256};
use sov_celestia_client::types::client_state::test_util::TendermintParamsConfig;
use sov_celestia_client::types::sovereign::SovereignParamsConfig;
use tokio::runtime::Builder;

#[test]
fn test_sovereign_to_cosmos() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

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

    let wasm_client_code_path =
        PathBuf::from(var("WASM_FILE_PATH").expect("Wasm file is required"));

    let cosmos_bootstrap = Arc::new(CosmosWithWasmClientBootstrap {
        runtime: runtime.clone(),
        builder: builder.clone(),
        should_randomize_identifiers: true,
        chain_store_dir: format!("./test-data/{store_postfix}/chains").into(),
        chain_command_path: "simd".into(),
        account_prefix: "sov".into(),
        staking_denom: "stake".into(),
        transfer_denom: "coin".into(),
        wasm_client_code_path: wasm_client_code_path.clone(),
    });

    let celestia_bootstrap = CelestiaBootstrap {
        runtime: runtime.clone(),
        builder: builder.clone(),
        chain_store_dir: store_dir.join("chains"),
        bridge_store_dir: store_dir.join("bridges"),
    };

    let node_binary = var("ROLLUP_PATH")
        .unwrap_or_else(|_| "rollup".to_string())
        .into();

    let sovereign_bootstrap = SovereignBootstrap {
        runtime: runtime.clone(),
        rollup_store_dir: store_dir.join("rollups"),
        rollup_command_path: node_binary,
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

        let sovereign_client_id = {
            let create_client_settings = ClientSettings::Tendermint(Settings {
                max_clock_drift: Duration::from_secs(40),
                trusting_period: None,
                trust_threshold: TrustThreshold::ONE_THIRD,
            });

            SovereignToCosmosRelay::create_client(
                SourceTarget,
                &sovereign_chain,
                cosmos_chain,
                &create_client_settings,
                &(),
            )
            .await?
        };

        println!(
            "client ID of Cosmos on Sovereign: {:?}",
            sovereign_client_id
        );

        let cosmos_client_id = {
            let wasm_client_bytes = tokio::fs::read(&wasm_client_code_path).await?;

            let wasm_code_hash: [u8; 32] = {
                let mut hasher = Sha256::new();
                hasher.update(wasm_client_bytes);
                hasher.finalize().into()
            };

            let rollup_genesis_da_height =
                Height::new(0, rollup_driver.node_config.runner.genesis_height)?;

            let sovereign_params = SovereignParamsConfig::builder()
                .genesis_da_height(rollup_genesis_da_height)
                .latest_height(Height::min(0)) // dummy value; overwritten by rollup latest height while creating client payload
                .build();

            let celestia_chain_id = celestia_chain_driver.chain().chain_id();

            let tendermint_params = TendermintParamsConfig::builder()
                .chain_id(celestia_chain_id.to_string().parse()?)
                .build();

            let create_client_settings = SovereignCreateClientOptions {
                tendermint_params_config: tendermint_params,
                sovereign_client_params: sovereign_params,
                code_hash: wasm_code_hash.into(),
            };

            SovereignToCosmosRelay::create_client(
                DestinationTarget,
                cosmos_chain,
                &sovereign_chain,
                &create_client_settings,
                &(),
            )
            .await?
        };

        println!("client ID of Sovereign on Cosmos: {:?}", cosmos_client_id);

        let sovereign_to_cosmos_relay = SovereignToCosmosRelay {
            runtime: runtime.clone(),
            src_chain: sovereign_chain.clone(),
            dst_chain: cosmos_chain.clone(),
            src_client_id: sovereign_client_id.clone(),
            dst_client_id: cosmos_client_id.clone(),
            packet_lock_mutex: Arc::new(Mutex::new(BTreeSet::new())),
        };

        let (connection_id_a, connection_id_b) = sovereign_to_cosmos_relay
            .bootstrap_connection(&Default::default())
            .await?;

        println!(
            "connection id on Sovereign: {}, connection id on Cosmos: {}",
            connection_id_a, connection_id_b
        );

        // FIXME: run bootstrap channel test
        let (channel_id_a, channel_b) = sovereign_to_cosmos_relay
            .bootstrap_channel(
                &PortId::transfer(),
                &PortId::transfer(),
                &CosmosInitChannelOptions::new(connection_id_a),
            )
            .await?;

        println!(
            "channel id on Sovereign: {}, channel id on Cosmos: {}",
            channel_id_a, channel_b
        );

        let wallet_b = &cosmos_chain_driver.user_wallet_a;

        let _address_b = &wallet_b.address;

        let wallet_a = rollup_driver.wallets.get("user-a").unwrap();

        let address_a = &wallet_a.address.address;

        let denom_a = &rollup_driver.genesis_config.transfer_token_address.address;

        let _balance_a1 = rollup.query_balance(address_a, denom_a).await?;

        // let packet = <SovereignRollup as CanIbcTransferToken<CosmosChain>>::ibc_transfer_token(
        //     &cosmos_chain,
        //     &channel_id_a,
        //     &PortId::transfer(),
        //     wallet_a,
        //     address_b,
        //     &Amount::new(1000, denom_a.clone()),
        // )
        // .await?;

        // println!("packet for IBC transfer from Cosmos: {}", packet);

        // sovereign_to_cosmos_relay.relay_packet(&packet).await?;

        <Result<(), Error>>::Ok(())
    })?;

    Ok(())
}
