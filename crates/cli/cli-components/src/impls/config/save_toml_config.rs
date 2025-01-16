use cgp::core::error::CanRaiseAsyncError;
use cgp::core::Async;
use hermes_runtime_components::traits::fs::write_file::CanWriteStringToFile;
use hermes_runtime_components::traits::runtime::HasRuntime;
use serde::Serialize;

use crate::traits::config::config_path::HasConfigPath;
use crate::traits::config::write_config::ConfigWriter;
use crate::traits::types::config::HasConfigType;

pub struct WriteTomlConfig;

impl<App, Runtime, Config> ConfigWriter<App> for WriteTomlConfig
where
    App: HasRuntime<Runtime = Runtime>
        + HasConfigType<Config = Config>
        + HasConfigPath
        + CanRaiseAsyncError<Runtime::Error>
        + CanRaiseAsyncError<toml::ser::Error>,
    Runtime: CanWriteStringToFile,
    Config: Async + Serialize,
{
    async fn write_config(app: &App, config: &App::Config) -> Result<(), App::Error> {
        let config_str = toml::to_string_pretty(config).map_err(App::raise_error)?;

        let config_path = app.config_path();

        app.runtime()
            .write_string_to_file(config_path, &config_str)
            .await
            .map_err(App::raise_error)?;

        Ok(())
    }
}
