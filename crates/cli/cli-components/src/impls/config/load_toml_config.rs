use cgp::prelude::*;
use hermes_core::runtime_components::traits::{CanReadFileAsString, HasRuntime};
use serde::de::DeserializeOwned;

use crate::traits::{ConfigLoader, ConfigLoaderComponent, HasConfigPath, HasConfigType};

pub struct LoadTomlConfig;

#[cgp_provider(ConfigLoaderComponent)]
impl<App, Runtime, Config> ConfigLoader<App> for LoadTomlConfig
where
    App: HasRuntime<Runtime = Runtime>
        + HasConfigType<Config = Config>
        + HasConfigPath
        + CanRaiseAsyncError<Runtime::Error>
        + CanRaiseAsyncError<toml::de::Error>,
    Runtime: CanReadFileAsString,
    Config: DeserializeOwned,
{
    async fn load_config(app: &App) -> Result<App::Config, App::Error> {
        let config_path = app.config_path();

        let config_str = app
            .runtime()
            .read_file_as_string(config_path)
            .await
            .map_err(App::raise_error)?;

        let config = toml::from_str(&config_str).map_err(App::raise_error)?;

        Ok(config)
    }
}
