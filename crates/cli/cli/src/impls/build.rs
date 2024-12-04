use hermes_cli_components::traits::build::{BuilderLoader, HasBuilderType};
use hermes_cli_components::traits::config::load_config::CanLoadConfig;
use hermes_cli_components::traits::types::config::HasConfigType;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::HasRuntime;
use ibc_relayer::config::{ChainConfig, Config};

pub struct LoadCosmosBuilder;

impl<App> BuilderLoader<App> for LoadCosmosBuilder
where
    App: HasBuilderType<Builder = CosmosBuilder>
        + HasConfigType<Config = Config>
        + HasRuntime<Runtime = HermesRuntime>
        + CanLoadConfig,
{
    async fn load_builder(app: &App) -> Result<App::Builder, App::Error> {
        let runtime = app.runtime().clone();
        let config = app.load_config().await?;

        let chain_configs = config
            .chains
            .into_iter()
            .map(|config| {
                let ChainConfig::CosmosSdk(config) = config;
                config.into()
            })
            .collect();

        let builder = CosmosBuilder::new(
            chain_configs,
            runtime,
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
        );

        Ok(builder)
    }
}
