#![recursion_limit = "256"]

use std::env::var;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;
use hermes_cosmos_relayer::types::error::Error;
use hermes_cosmos_wasm_relayer::context::cosmos_bootstrap::CosmosWithWasmClientBootstrap;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_test_components::bootstrap::traits::chain::CanBootstrapChain;
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

    let cosmos_bootstrap = Arc::new(CosmosWithWasmClientBootstrap {
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
        let _cosmos_chain_driver = cosmos_bootstrap.bootstrap_chain("cosmos").await?;

        Ok(())
    })
}
