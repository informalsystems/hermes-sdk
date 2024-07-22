use core::fmt::Debug;
use std::path::PathBuf;

use cgp_core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp_core::prelude::*;
use cgp_error_eyre::{ProvideEyreError, RaiseDebugError};
use hermes_any_counterparty::contexts::any_counterparty::AnyCounterparty;
use hermes_cli_components::impls::commands::delegate::DelegateCommandRunner;
use hermes_cli_components::impls::commands::queries::client_state::{
    QueryClientStateArgs, RunQueryClientStateCommand,
};
use hermes_cli_components::impls::get_config_path::GetDefaultConfigField;
use hermes_cli_components::impls::load_toml_config::LoadTomlConfig;
use hermes_cli_components::impls::parse::delegate::DelegateArgParsers;
use hermes_cli_components::impls::parse::string::ParseFromString;
use hermes_cli_components::traits::any_counterparty::ProvideAnyCounterparty;
use hermes_cli_components::traits::build::{
    BuilderLoaderComponent, CanLoadBuilder, ProvideBuilderType,
};
use hermes_cli_components::traits::command::{CanRunCommand, CommandRunnerComponent};
use hermes_cli_components::traits::config::config_path::ConfigPathGetterComponent;
use hermes_cli_components::traits::config::load_config::{CanLoadConfig, ConfigLoaderComponent};
use hermes_cli_components::traits::output::{HasOutputType, OutputProducer, ProvideOutputType};
use hermes_cli_components::traits::parse::{ArgParserComponent, CanParseArg};
use hermes_cli_components::traits::types::config::ProvideConfigType;
use hermes_cli_framework::output::Output;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
use hermes_error::types::HermesError;
use hermes_logger::ProvideHermesLogger;
use hermes_logging_components::traits::has_logger::{
    GlobalLoggerGetterComponent, LoggerGetterComponent, LoggerTypeComponent,
};
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{
    ProvideDefaultRuntimeField, RuntimeGetterComponent, RuntimeTypeComponent,
};
use ibc_relayer::config::Config;
use ibc_relayer_types::core::ics02_client::height::Height;
use ibc_relayer_types::core::ics24_host::identifier::{ChainId, ClientId};
use serde::Serialize;

use crate::impls::build::LoadCosmosBuilder;
use crate::impls::parse_height::ParseCosmosHeight;

#[derive(HasField)]
pub struct HermesApp {
    pub config_path: PathBuf,
    pub runtime: HermesRuntime,
}

pub struct HermesAppComponents;

pub struct HermesParserComponents;

pub struct HermesCommandRunnerComponents;

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
        [
            LoggerTypeComponent,
            LoggerGetterComponent,
            GlobalLoggerGetterComponent,
        ]:
            ProvideHermesLogger,
        ConfigPathGetterComponent:
            GetDefaultConfigField,
        ConfigLoaderComponent:
            LoadTomlConfig,
        BuilderLoaderComponent:
            LoadCosmosBuilder,
        ArgParserComponent:
            DelegateArgParsers<HermesParserComponents>,
        CommandRunnerComponent:
            DelegateCommandRunner<HermesCommandRunnerComponents>,
    }
}

delegate_components! {
    HermesParserComponents {
        (QueryClientStateArgs, symbol!("chain_id")): ParseFromString<ChainId>,
        (QueryClientStateArgs, symbol!("client_id")): ParseFromString<ClientId>,
        (QueryClientStateArgs, (symbol!("chain_id"), symbol!("height"))): ParseCosmosHeight,
    }
}

delegate_components! {
    HermesCommandRunnerComponents {
        QueryClientStateArgs: RunQueryClientStateCommand,
    }
}

impl<App> ProvideBuilderType<App> for HermesAppComponents
where
    App: Async,
{
    type Builder = CosmosBuilder;
}

impl<App> ProvideAnyCounterparty<App> for HermesAppComponents
where
    App: Async,
{
    type AnyCounterparty = AnyCounterparty;
}

impl<App> ProvideConfigType<App> for HermesAppComponents
where
    App: Async,
{
    type Config = Config;
}

impl<App> ProvideOutputType<App> for HermesAppComponents
where
    App: Async,
{
    type Output = Output;
}

impl<App, Value> OutputProducer<App, Value> for HermesAppComponents
where
    App: HasOutputType<Output = Output>,
    Value: Serialize + Debug + Async,
{
    fn produce_output(_app: &App, value: Value) -> Output {
        Output::success(value)
    }
}

pub trait CanUseHermesApp:
    CanLoadConfig
    + CanLoadBuilder
    + CanParseArg<QueryClientStateArgs, symbol!("chain_id"), Parsed = ChainId>
    + CanParseArg<QueryClientStateArgs, symbol!("client_id"), Parsed = ClientId>
    + CanParseArg<
        QueryClientStateArgs,
        (symbol!("chain_id"), symbol!("height")),
        Parsed = Option<Height>,
    > + CanRaiseError<HermesError>
    + CanRunCommand<QueryClientStateArgs>
{
}

impl CanUseHermesApp for HermesApp {}
