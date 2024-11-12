use alloc::sync::Arc;

use cgp::prelude::*;
use hermes_cli_components::traits::bootstrap::{BootstrapLoader, HasBootstrapType};
use hermes_cosmos_integration_tests::contexts::bootstrap::CosmosBootstrap;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
use hermes_error::types::HermesError;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::HasRuntime;
use ibc_relayer::config::dynamic_gas::DynamicGasPrice;

#[derive(Debug, clap::Parser, HasField)]
pub struct BootstrapChainArgs {
    #[clap(long = "chain-id", required = true)]
    pub chain_id: String,

    #[clap(long = "chain-store-dir", required = true)]
    pub chain_store_dir: String,

    #[clap(long = "chain-command-path", default_value = "gaiad")]
    pub chain_command_path: String,

    #[clap(long = "account-prefix", default_value = "cosmos")]
    pub account_prefix: String,

    #[clap(long = "staking-denom", default_value = "stake")]
    pub staking_denom: String,

    #[clap(long = "transfer-denom", default_value = "samoleon")]
    pub transfer_denom: String,
}

pub struct LoadCosmosBootstrap;

impl<App> BootstrapLoader<App, BootstrapChainArgs> for LoadCosmosBootstrap
where
    App: HasBootstrapType<Bootstrap = CosmosBootstrap>
        + HasRuntime<Runtime = HermesRuntime>
        + CanRaiseError<HermesError>,
{
    async fn load_bootstrap(
        app: &App,
        args: &BootstrapChainArgs,
    ) -> Result<App::Bootstrap, App::Error> {
        let dynamic_gas_fee_enabled = std::env::var("ENABLE_DYNAMIC_GAS")
            .map(|v| &v == "true")
            .unwrap_or(false);

        let runtime = app.runtime();

        let builder = CosmosBuilder::new_with_default(runtime.clone());

        let bootstrap = CosmosBootstrap {
            runtime: runtime.clone(),
            cosmos_builder: Arc::new(builder),
            should_randomize_identifiers: false,
            chain_store_dir: args.chain_store_dir.clone().into(),
            chain_command_path: args.chain_command_path.clone().into(),
            account_prefix: args.account_prefix.clone(),
            staking_denom_prefix: args.staking_denom.clone(),
            transfer_denom_prefix: args.transfer_denom.clone(),
            genesis_config_modifier: Box::new(|_| Ok(())),
            comet_config_modifier: Box::new(|_| Ok(())),
            dynamic_gas: DynamicGasPrice::unsafe_new(dynamic_gas_fee_enabled, 1.3, 1.6),
        };

        Ok(bootstrap)
    }
}
