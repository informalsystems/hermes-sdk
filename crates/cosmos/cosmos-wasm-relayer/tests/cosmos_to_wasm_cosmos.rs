#![recursion_limit = "256"]

use core::time::Duration;
use std::env::var;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use hermes_cosmos_chain_components::types::payloads::client::CosmosCreateClientOptions;
use hermes_cosmos_integration_tests::contexts::bootstrap::CosmosBootstrap;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
use hermes_cosmos_wasm_relayer::context::chain::WasmCosmosChain;
use hermes_cosmos_wasm_relayer::context::cosmos_bootstrap::CosmosWithWasmClientBootstrap;
use hermes_cosmos_wasm_relayer::context::cosmos_to_wasm_cosmos_relay::CosmosToWasmCosmosRelay;
use hermes_cosmos_wasm_relayer::types::create_client::CreateWasmTendermintMessageOptions;
use hermes_error::types::Error;
use hermes_relayer_components::relay::traits::client_creator::CanCreateClient;
use hermes_relayer_components::relay::traits::target::{DestinationTarget, SourceTarget};
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_test_components::bootstrap::traits::chain::CanBootstrapChain;
use ibc_proto::ibc::lightclients::tendermint::v1::Fraction;
use sha2::{Digest, Sha256};
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

    let gaia_bootstrap = Arc::new(CosmosBootstrap {
        runtime: runtime.clone(),
        cosmos_builder: builder.clone(),
        should_randomize_identifiers: true,
        chain_store_dir: store_dir.join("chains"),
        chain_command_path: "simd".into(),
        account_prefix: "cosmos".into(),
        staking_denom_prefix: "stake".into(),
        transfer_denom_prefix: "coin".into(),
        genesis_config_modifier: Box::new(|_| Ok(())),
        comet_config_modifier: Box::new(|_| Ok(())),
        dynamic_gas: None,
    });

    tokio_runtime.block_on(async move {
        let wasm_client_byte_code = tokio::fs::read(&wasm_client_code_path).await?;

        let wasm_code_hash: [u8; 32] = {
            let mut hasher = Sha256::new();
            hasher.update(&wasm_client_byte_code);
            hasher.finalize().into()
        };

        let simd_bootstrap = Arc::new(CosmosWithWasmClientBootstrap {
            runtime: runtime.clone(),
            cosmos_builder: builder.clone(),
            should_randomize_identifiers: true,
            chain_store_dir: store_dir.join("chains"),
            chain_command_path: "simd".into(),
            account_prefix: "cosmos".into(),
            staking_denom_prefix: "stake".into(),
            transfer_denom_prefix: "coin".into(),
            wasm_client_byte_code,
            governance_proposal_authority: "cosmos10d07y265gmmuvt4z0w9aw880jnsr700j6zn9kn".into(), // TODO: don't hard code this
            dynamic_gas: None,
        });

        let gaia_chain_driver = gaia_bootstrap.bootstrap_chain("gaia").await?;

        let simd_chain_driver = simd_bootstrap.bootstrap_chain("simd").await?;

        let simd_chain = simd_chain_driver.chain.clone();

        let gaia_chain = WasmCosmosChain {
            chain: gaia_chain_driver.chain.clone(),
        };

        let tm_create_client_settings = CosmosCreateClientOptions {
            max_clock_drift: Duration::from_secs(40),
            trust_threshold: Fraction {
                numerator: 1,
                denominator: 3,
            },
            ..Default::default()
        };

        let client_id_a = CosmosToWasmCosmosRelay::create_client(
            SourceTarget,
            &simd_chain,
            &gaia_chain,
            &tm_create_client_settings,
            &CreateWasmTendermintMessageOptions {
                code_hash: wasm_code_hash.into(),
            },
        )
        .await?;

        println!("client_id_a: {client_id_a}");

        let client_id_b = CosmosToWasmCosmosRelay::create_client(
            DestinationTarget,
            &gaia_chain,
            &simd_chain,
            &tm_create_client_settings,
            &(),
        )
        .await?;

        println!("client_id_b: {client_id_b}");

        let _relay: CosmosToWasmCosmosRelay = CosmosToWasmCosmosRelay::new(
            runtime.clone(),
            simd_chain,
            gaia_chain,
            client_id_a,
            client_id_b,
            Default::default(),
        );

        // FIXME: connection bootstrap currently fails at OpenTry,
        // due to bugs on ibc-go.

        // let (connection_id_a, connection_id_b) =
        //     relay.bootstrap_connection(&Default::default()).await?;

        // println!("successfully bootstrapped connections: {connection_id_a} <> {connection_id_b}");

        Ok(())
    })
}
