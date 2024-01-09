use core::time::Duration;
use std::sync::Arc;

use eyre::Error;
use hermes_cosmos_relayer::contexts::relay::CosmosRelay;
use hermes_relayer_components::relay::traits::components::client_creator::CanCreateClient;
use hermes_relayer_components::relay::traits::target::{DestinationTarget, SourceTarget};
use hermes_relayer_runtime::types::runtime::HermesRuntime;
use hermes_test_components::bootstrap::traits::chain::CanBootstrapChain;
use ibc_relayer::chain::client::ClientSettings;
use ibc_relayer::foreign_client::CreateOptions;
use tokio::runtime::Runtime;
use tokio::test;
use tokio::time::sleep;

use crate::contexts::bootstrap::CosmosStdBootstrapContext;

#[test(flavor = "multi_thread")]
async fn test_bootstrap_cosmos_chain() -> Result<(), Error> {
    stable_eyre::install()?;

    let runtime = Arc::new(Runtime::new()?);
    let runtime_context = HermesRuntime::new(runtime.clone());

    let bootstrap = CosmosStdBootstrapContext {
        runtime: runtime_context,
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

    let _client_a_to_b_id = CosmosRelay::create_client(
        SourceTarget,
        &chain_a.base_chain,
        &chain_b.base_chain,
        &ClientSettings::for_create_command(
            CreateOptions::default(),
            &chain_a.chain_config,
            &chain_b.chain_config,
        ),
    )
    .await?;

    let _client_b_to_a_id = CosmosRelay::create_client(
        DestinationTarget,
        &chain_b.base_chain,
        &chain_a.base_chain,
        &ClientSettings::for_create_command(
            CreateOptions::default(),
            &chain_b.chain_config,
            &chain_a.chain_config,
        ),
    )
    .await?;

    Ok(())
}
