use std::sync::Arc;

use eyre::Error;
use hermes_relayer_runtime::types::runtime::HermesRuntime;
use hermes_test_components::bootstrap::traits::chain::CanBootstrapChain;
use tokio::runtime::Runtime;

use crate::contexts::bootstrap::CosmosStdBootstrapContext;

#[test]
fn test_bootstrap_cosmos_chain() -> Result<(), Error> {
    stable_eyre::install()?;

    let runtime = Arc::new(Runtime::new()?);
    let runtime_context = HermesRuntime::new(runtime.clone());

    let bootstrap = CosmosStdBootstrapContext {
        runtime: runtime_context,
        should_randomize_identifiers: false,
        test_dir: "./test-data".into(),
        chain_command_path: "gaiad".into(),
        genesis_config_modifier: Box::new(|_| Ok(())),
        comet_config_modifier: Box::new(|_| Ok(())),
    };

    runtime.block_on(async move {
        bootstrap.bootstrap_chain("cosmos-testnet-1").await.unwrap();
    });

    Ok(())
}
