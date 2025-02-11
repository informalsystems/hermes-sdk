use cgp::prelude::*;
use hermes_cli_components::traits::build::{BuilderLoader, BuilderLoaderComponent, HasBuilderType};
use hermes_cli_components::traits::config::load_config::CanLoadConfig;
use hermes_cli_components::traits::types::config::HasConfigType;
use hermes_cosmos_chain_components::impls::types::config::RelayerConfig;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::HasRuntime;

pub struct LoadCosmosBuilder;

#[cgp_provider(BuilderLoaderComponent)]
impl<App> BuilderLoader<App> for LoadCosmosBuilder
where
    App: HasBuilderType<Builder = CosmosBuilder>
        + HasConfigType<Config = RelayerConfig>
        + HasRuntime<Runtime = HermesRuntime>
        + CanLoadConfig,
{
    async fn load_builder(app: &App) -> Result<App::Builder, App::Error> {
        let runtime = app.runtime().clone();
        let config = app.load_config().await?;

        let builder = CosmosBuilder::new(
            config.chains,
            runtime,
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
        );

        Ok(builder)
    }
}
