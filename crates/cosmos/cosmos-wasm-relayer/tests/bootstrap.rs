#![recursion_limit = "256"]

use core::time::Duration;
use std::env::var;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use hermes_cosmos_integration_tests::contexts::bootstrap::CosmosBootstrap;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;
use hermes_cosmos_relayer::types::error::Error;
use hermes_cosmos_wasm_relayer::context::chain::WasmCosmosChain;
use hermes_cosmos_wasm_relayer::context::cosmos_bootstrap::CosmosWithWasmClientBootstrap;
use hermes_cosmos_wasm_relayer::context::cosmos_to_wasm_cosmos_relay::CosmosToWasmCosmosRelay;
use hermes_relayer_components::relay::traits::client_creator::CanCreateClient;
use hermes_relayer_components::relay::traits::target::{DestinationTarget, SourceTarget};
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_test_components::bootstrap::traits::chain::CanBootstrapChain;
use ibc_relayer::chain::client::ClientSettings;
use ibc_relayer::chain::cosmos::client::Settings;
use ibc_relayer::config::types::TrustThreshold;
use tokio::runtime::Builder;

#[test]
fn test_cosmos_to_wasm_cosmos() -> Result<(), Error> {
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

    let cosmos_bootstrap = Arc::new(CosmosBootstrap {
        runtime: runtime.clone(),
        builder: builder.clone(),
        should_randomize_identifiers: true,
        chain_store_dir: store_dir.join("chains"),
        chain_command_path: "gaiad".into(),
        account_prefix: "cosmos".into(),
        staking_denom: "stake".into(),
        transfer_denom: "coin".into(),
        genesis_config_modifier: Box::new(|_| Ok(())),
        comet_config_modifier: Box::new(|_| Ok(())),
    });

    let wasm_cosmos_bootstrap = Arc::new(CosmosWithWasmClientBootstrap {
        runtime: runtime.clone(),
        builder: builder.clone(),
        should_randomize_identifiers: true,
        chain_store_dir: store_dir.join("chains"),
        chain_command_path: "simd".into(),
        account_prefix: "cosmos".into(),
        staking_denom: "stake".into(),
        transfer_denom: "coin".into(),
        wasm_client_code_path: wasm_client_code_path.clone(),
    });

    tokio_runtime.block_on(async move {
        let cosmos_chain_driver = cosmos_bootstrap.bootstrap_chain("cosmos").await?;

        let cosmos_chain = cosmos_chain_driver.chain.clone();

        let wasm_cosmos_chain_driver = wasm_cosmos_bootstrap.bootstrap_chain("wasmcosmos").await?;

        let wasm_cosmos_chain = WasmCosmosChain {
            chain: wasm_cosmos_chain_driver.chain.clone(),
        };

        let create_client_settings = ClientSettings::Tendermint(Settings {
            max_clock_drift: Duration::from_secs(40),
            trusting_period: None,
            trust_threshold: TrustThreshold::ONE_THIRD,
        });

        let client_id_a = CosmosToWasmCosmosRelay::create_client(
            SourceTarget,
            &cosmos_chain,
            &wasm_cosmos_chain,
            &create_client_settings,
        )
        .await?;

        // TODO: define a custom CreateClientOptions for WasmCosmos

        let client_id_b = CosmosToWasmCosmosRelay::create_client(
            DestinationTarget,
            &wasm_cosmos_chain,
            &cosmos_chain,
            &create_client_settings,
        )
        .await?;

        let _relay = CosmosToWasmCosmosRelay::new(
            runtime.clone(),
            cosmos_chain,
            wasm_cosmos_chain,
            client_id_a,
            client_id_b,
            Default::default(),
        );

        // let (connection_id_a, connection_id_b) = relay.bootstrap_connection().await?;

        Ok(())
    })
}
