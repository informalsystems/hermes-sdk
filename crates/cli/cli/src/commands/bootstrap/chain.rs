use alloc::sync::Arc;

use hermes_cli_components::traits::{BootstrapLoader, BootstrapLoaderComponent, HasBootstrapType};
use hermes_core::runtime_components::traits::HasRuntime;
use hermes_cosmos::chain_components::types::DynamicGasConfig;
use hermes_cosmos::error::types::HermesError;
use hermes_cosmos::integration_tests::contexts::{CosmosBootstrap, CosmosBootstrapFields};
use hermes_cosmos::relayer::contexts::CosmosBuilder;
use hermes_cosmos::runtime::types::runtime::HermesRuntime;
use hermes_prelude::*;

#[derive(Debug, clap::Parser, HasField)]
pub struct BootstrapCosmosChainArgs {
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

#[cgp_provider(BootstrapLoaderComponent)]
impl<App, Tag> BootstrapLoader<App, Tag, BootstrapCosmosChainArgs> for LoadCosmosBootstrap
where
    App: HasBootstrapType<Tag, Bootstrap = CosmosBootstrap>
        + HasRuntime<Runtime = HermesRuntime>
        + CanRaiseAsyncError<HermesError>,
{
    async fn load_bootstrap(
        app: &App,
        args: &BootstrapCosmosChainArgs,
    ) -> Result<App::Bootstrap, App::Error> {
        let runtime = app.runtime();

        let builder = CosmosBuilder::new_with_default(runtime.clone());

        let bootstrap = CosmosBootstrap {
            fields: Arc::new(CosmosBootstrapFields {
                runtime: runtime.clone(),
                cosmos_builder: builder,
                should_randomize_identifiers: false,
                chain_store_dir: args.chain_store_dir.clone().into(),
                chain_command_path: args.chain_command_path.clone().into(),
                account_prefix: args.account_prefix.clone(),
                staking_denom_prefix: args.staking_denom.clone(),
                transfer_denom_prefix: args.transfer_denom.clone(),
                genesis_config_modifier: Box::new(|_| Ok(())),
                comet_config_modifier: Box::new(|_| Ok(())),
                dynamic_gas: Some(DynamicGasConfig::default()),
            }),
        };

        Ok(bootstrap)
    }
}
