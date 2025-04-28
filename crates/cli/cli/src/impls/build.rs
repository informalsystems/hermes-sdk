use hermes_cli_components::traits::{
    BuilderLoader, BuilderLoaderComponent, CanLoadConfig, HasBuilderType, HasConfigType,
};
use hermes_core::runtime_components::traits::HasRuntime;
use hermes_cosmos::chain_components::impls::RelayerConfig;
use hermes_cosmos::relayer::contexts::CosmosBuilder;
use hermes_cosmos::runtime::types::runtime::HermesRuntime;
use hermes_prelude::*;

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
