#![recursion_limit = "256"]

use std::env::var;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use hermes_core::relayer_components::relay::traits::{
    CanCreateClient, DestinationTarget, SourceTarget,
};
use hermes_core::test_components::bootstrap::traits::CanBootstrapChain;
use hermes_cosmos_relayer::contexts::CosmosBuilder;
use hermes_cosmos_wasm_relayer::context::{
    CosmosWithWasmClientBootstrap, WasmCosmosChain, WasmCosmosRelay,
};
use hermes_cosmos_wasm_relayer::types::CreateWasmTendermintMessageOptions;
use hermes_error::types::Error;
use hermes_runtime::types::runtime::HermesRuntime;
use sha2::{Digest, Sha256};
use tokio::runtime::Builder;

#[test]
fn test_both_wasm_cosmos() -> Result<(), Error> {
    let _ = stable_eyre::install();

    let tokio_runtime = Arc::new(Builder::new_multi_thread().enable_all().build()?);

    let runtime = HermesRuntime::new(tokio_runtime.clone());

    let builder = CosmosBuilder::new_with_default(runtime.clone());

    let store_postfix = format!(
        "{}-{}",
        SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis(),
        rand::random::<u64>()
    );

    let store_dir = std::env::current_dir()?.join(format!("test-data/{store_postfix}"));

    let wasm_client_code_path =
        PathBuf::from(var("WASM_FILE_PATH").expect("Wasm file is required"));

    tokio_runtime.block_on(async move {
        let wasm_client_byte_code = tokio::fs::read(&wasm_client_code_path).await?;

        let wasm_code_hash: [u8; 32] = {
            let mut hasher = Sha256::new();
            hasher.update(&wasm_client_byte_code);
            hasher.finalize().into()
        };

        let wasm_additional_byte_code = match var("ADDITIONAL_WASM_FILE_PATH") {
            Ok(paths_str) => paths_str
                .split(',')
                .map(PathBuf::from)
                .map(std::fs::read)
                .collect::<Result<Vec<_>, _>>()?,
            Err(_) => vec![],
        };

        let bootstrap = Arc::new(CosmosWithWasmClientBootstrap {
            runtime: runtime.clone(),
            cosmos_builder: builder.clone(),
            should_randomize_identifiers: true,
            chain_store_dir: store_dir.join("chains"),
            chain_command_path: "simd".into(),
            account_prefix: "cosmos".into(),
            staking_denom_prefix: "stake".into(),
            transfer_denom_prefix: "coin".into(),
            wasm_client_byte_code,
            wasm_additional_byte_code,
            governance_proposal_authority: "cosmos10d07y265gmmuvt4z0w9aw880jnsr700j6zn9kn".into(), // TODO: don't hard code this
            dynamic_gas: None,
        });

        let chain_driver_a = bootstrap.bootstrap_chain("chain-a").await?;

        let chain_driver_b = bootstrap.bootstrap_chain("chain-b").await?;

        let chain_a = WasmCosmosChain {
            chain: chain_driver_a.chain.clone(),
        };

        let chain_b = WasmCosmosChain {
            chain: chain_driver_b.chain.clone(),
        };

        let client_id_a = WasmCosmosRelay::create_client(
            SourceTarget,
            &chain_a,
            &chain_b,
            &Default::default(),
            &CreateWasmTendermintMessageOptions {
                code_hash: wasm_code_hash.into(),
            },
        )
        .await?;

        println!("client_id_a: {client_id_a}");

        let client_id_b = WasmCosmosRelay::create_client(
            DestinationTarget,
            &chain_b,
            &chain_a,
            &Default::default(),
            &CreateWasmTendermintMessageOptions {
                code_hash: wasm_code_hash.into(),
            },
        )
        .await?;

        println!("client_id_b: {client_id_b}");

        let _relay = WasmCosmosRelay::new(
            runtime.clone(),
            chain_a,
            chain_b,
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
