use cgp::core::error::CanRaiseAsyncError;
use hermes_runtime_components::traits::fs::read_file::CanReadFileAsString;
use hermes_runtime_components::traits::runtime::HasRuntime;
use serde::de::DeserializeOwned;

use crate::traits::config::config_path::HasConfigPath;
use crate::traits::config::load_config::ConfigLoader;
use crate::traits::types::config::HasConfigType;

pub struct LoadTomlConfig;

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
