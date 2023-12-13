use alloc::sync::Arc;
use cosmos_test_components::chain::types::denom::Denom;
use eyre::Error;
use ibc_relayer_runtime::types::runtime::TokioRuntimeContext;
use ibc_test_components::bootstrap::traits::chain::CanBootstrapChain;
use tokio::runtime::Runtime;

use crate::contexts::bootstrap::CosmosStdBootstrapContext;

#[test]
fn test_bootstrap_cosmos_chain() -> Result<(), Error> {
    stable_eyre::install()?;

    let runtime = Arc::new(Runtime::new()?);
    let runtime_context = TokioRuntimeContext::new(runtime.clone());

    let bootstrap = CosmosStdBootstrapContext {
        runtime: runtime_context,
        should_randomize_identifiers: false,
        test_dir: "./test-data".into(),
        chain_command_path: "gaiad".into(),
        staking_denom: Denom::base("stake"),
        transfer_denom: Denom::base("coin"),
        genesis_config_modifier: Box::new(|_| Ok(())),
        comet_config_modifier: Box::new(|_| Ok(())),
    };

    runtime.block_on(async move {
        bootstrap.bootstrap_chain("cosmos-testnet-1").await.unwrap();
    });

    Ok(())
}
