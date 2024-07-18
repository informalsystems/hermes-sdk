use std::path::PathBuf;

use cgp_core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp_core::prelude::*;
use cgp_error_eyre::{ProvideEyreError, RaiseDebugError};
use hermes_cli_components::impls::get_config_path::GetDefaultConfigField;
use hermes_cli_components::impls::load_toml_config::LoadTomlConfig;
use hermes_cli_components::traits::build::{
    BuilderLoaderComponent, CanLoadBuilder, ProvideBuilderType,
};
use hermes_cli_components::traits::config::config_path::ConfigPathGetterComponent;
use hermes_cli_components::traits::config::load_config::{CanLoadConfig, ConfigLoaderComponent};
use hermes_cli_components::traits::types::config::ProvideConfigType;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{
    ProvideDefaultRuntimeField, RuntimeGetterComponent, RuntimeTypeComponent,
};
use ibc_relayer::config::Config;

use crate::impls::build::LoadCosmosBuilder;

#[derive(HasField)]
pub struct HermesApp {
    pub config_path: PathBuf,
    pub runtime: HermesRuntime,
}

pub struct HermesAppComponents;

impl HasComponents for HermesApp {
    type Components = HermesAppComponents;
}

delegate_components! {
    HermesAppComponents {
        ErrorTypeComponent: ProvideEyreError,
        ErrorRaiserComponent: RaiseDebugError,
        [
            RuntimeTypeComponent,
            RuntimeGetterComponent,
        ]:
            ProvideDefaultRuntimeField,
        ConfigPathGetterComponent:
            GetDefaultConfigField,
        ConfigLoaderComponent:
            LoadTomlConfig,
        BuilderLoaderComponent:
            LoadCosmosBuilder,
    }
}

impl<App> ProvideBuilderType<App> for HermesAppComponents
where
    App: Async,
{
    type Builder = CosmosBuilder;
}

impl<App> ProvideConfigType<App> for HermesAppComponents
where
    App: Async,
{
    type Config = Config;
}

pub trait CanUseHermesApp: CanLoadConfig + CanLoadBuilder {}

impl CanUseHermesApp for HermesApp {}
