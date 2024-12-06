use core::fmt::Debug;
use std::path::PathBuf;

use cgp::core::component::{UseContext, UseDelegate};
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp::core::types::impls::WithType;
use cgp::prelude::*;
use hermes_any_counterparty::contexts::any_counterparty::AnyCounterparty;
use hermes_cli_components::impls::commands::bootstrap::chain::RunBootstrapChainCommand;
use hermes_cli_components::impls::commands::client::create::{
    CreateClientOptionsParser, RunCreateClientCommand,
};
use hermes_cli_components::impls::commands::queries::client::{
    QueryClientSubCommand, RunQueryClientSubCommand,
};
use hermes_cli_components::impls::commands::queries::client_state::{
    QueryClientStateArgs, RunQueryClientStateCommand,
};
use hermes_cli_components::impls::commands::queries::client_status::{
    QueryClientStatusArgs, RunQueryClientStatusCommand,
};
use hermes_cli_components::impls::commands::queries::connection::{
    QueryConnectionSubCommand, RunQueryConnectionSubCommand,
};
use hermes_cli_components::impls::commands::queries::connection_end::{
    QueryConnectionEndArgs, RunQueryConnectionEndCommand,
};
use hermes_cli_components::impls::commands::queries::consensus_state::{
    QueryConsensusStateArgs, RunQueryConsensusStateCommand,
};
use hermes_cli_components::impls::commands::start::{RunStartRelayerCommand, StartRelayerArgs};
use hermes_cli_components::impls::config::get_config_path::GetDefaultConfigField;
use hermes_cli_components::impls::config::load_toml_config::LoadTomlConfig;
use hermes_cli_components::impls::config::save_toml_config::WriteTomlConfig;
use hermes_cli_components::impls::parse::string::{ParseFromOptionalString, ParseFromString};
use hermes_cli_components::traits::any_counterparty::ProvideAnyCounterparty;
use hermes_cli_components::traits::bootstrap::{BootstrapLoaderComponent, BootstrapTypeComponent};
use hermes_cli_components::traits::build::{
    BuilderLoaderComponent, BuilderTypeComponent, CanLoadBuilder,
};
use hermes_cli_components::traits::command::{CanRunCommand, CommandRunnerComponent};
use hermes_cli_components::traits::config::config_path::ConfigPathGetterComponent;
use hermes_cli_components::traits::config::load_config::{CanLoadConfig, ConfigLoaderComponent};
use hermes_cli_components::traits::config::write_config::{CanWriteConfig, ConfigWriterComponent};
use hermes_cli_components::traits::output::{
    CanProduceOutput, HasOutputType, OutputProducer, OutputTypeComponent,
};
use hermes_cli_components::traits::parse::ArgParserComponent;
use hermes_cli_components::traits::types::config::ConfigTypeComponent;
use hermes_cli_framework::output::Output;
use hermes_cosmos_chain_components::types::payloads::client::CosmosCreateClientOptions;
use hermes_cosmos_integration_tests::contexts::bootstrap::CosmosBootstrap;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_error::traits::wrap::WrapError;
use hermes_error::types::{Error, HermesError};
use hermes_logger::ProvideHermesLogger;
use hermes_logging_components::traits::has_logger::{
    GlobalLoggerGetterComponent, LoggerGetterComponent, LoggerTypeComponent,
};
use hermes_relayer_components::error::traits::retry::RetryableErrorComponent;
use hermes_relayer_components::multi::types::index::Index;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{
    ProvideDefaultRuntimeField, RuntimeGetterComponent, RuntimeTypeComponent,
};
use ibc::core::client::types::Height;
use ibc::core::host::types::identifiers::{ChainId, ClientId, ConnectionId};
use ibc_relayer::config::Config;
use serde::Serialize;

use crate::commands::bootstrap::chain::{BootstrapChainArgs, LoadCosmosBootstrap};
use crate::commands::bootstrap::subcommand::{BootstrapSubCommand, RunBootstrapSubCommand};
use crate::commands::client::create::CreateClientArgs;
use crate::impls::build::LoadCosmosBuilder;
use crate::impls::error::ProvideCliError;

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
        [
            ErrorTypeComponent,
            ErrorRaiserComponent,
            RetryableErrorComponent,
        ]:
            ProvideCliError,
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
        ConfigTypeComponent:
            WithType<Config>,
        BootstrapTypeComponent:
            WithType<CosmosBootstrap>,
        BuilderTypeComponent:
            WithType<CosmosBuilder>,
        OutputTypeComponent:
            WithType<Output>,
        ConfigPathGetterComponent:
            GetDefaultConfigField,
        ConfigLoaderComponent:
            LoadTomlConfig,
        ConfigWriterComponent:
            WriteTomlConfig,
        BuilderLoaderComponent:
            LoadCosmosBuilder,
        BootstrapLoaderComponent:
            LoadCosmosBootstrap,
        ArgParserComponent:
            UseDelegate<HermesParserComponents>,
        CommandRunnerComponent:
            UseDelegate<HermesCommandRunnerComponents>,
    }
}

delegate_components! {
    HermesParserComponents {
        (QueryConnectionEndArgs, symbol!("chain_id")): ParseFromString<ChainId>,
        (QueryConnectionEndArgs, symbol!("connection_id")): ParseFromString<ConnectionId>,
        (QueryConnectionEndArgs, symbol!("height")): ParseFromOptionalString<Height>,

        (QueryClientStateArgs, symbol!("chain_id")): ParseFromString<ChainId>,
        (QueryClientStateArgs, symbol!("client_id")): ParseFromString<ClientId>,
        (QueryClientStateArgs, symbol!("height")): ParseFromOptionalString<Height>,

        (QueryConsensusStateArgs, symbol!("chain_id")): ParseFromString<ChainId>,
        (QueryConsensusStateArgs, symbol!("client_id")): ParseFromString<ClientId>,
        (QueryConsensusStateArgs, symbol!("query_height")): ParseFromOptionalString<Height>,
        (QueryConsensusStateArgs, symbol!("consensus_height")): ParseFromOptionalString<Height>,

        (StartRelayerArgs, symbol!("chain_id_a")): ParseFromString<ChainId>,
        (StartRelayerArgs, symbol!("client_id_a")): ParseFromString<ClientId>,
        (StartRelayerArgs, symbol!("chain_id_b")): ParseFromString<ChainId>,
        (StartRelayerArgs, symbol!("client_id_b")): ParseFromString<ClientId>,

        (QueryClientStatusArgs, symbol!("chain_id")): ParseFromString<ChainId>,
        (QueryClientStatusArgs, symbol!("client_id")): ParseFromString<ClientId>,

        (CreateClientArgs, symbol!("target_chain_id")): ParseFromString<ChainId>,
        (CreateClientArgs, symbol!("counterparty_chain_id")): ParseFromString<ChainId>,
    }
}

delegate_components! {
    HermesCommandRunnerComponents {
        StartRelayerArgs: RunStartRelayerCommand,

        QueryClientSubCommand: RunQueryClientSubCommand,
        QueryClientStateArgs: RunQueryClientStateCommand,
        QueryClientStatusArgs: RunQueryClientStatusCommand,
        QueryConsensusStateArgs: RunQueryConsensusStateCommand,

        CreateClientArgs: RunCreateClientCommand,

        BootstrapSubCommand: RunBootstrapSubCommand,
        BootstrapChainArgs: RunBootstrapChainCommand<UseContext>,

        QueryConnectionSubCommand: RunQueryConnectionSubCommand,
        QueryConnectionEndArgs: RunQueryConnectionEndCommand,
    }
}

impl<App> ProvideAnyCounterparty<App> for HermesAppComponents
where
    App: Async,
{
    type AnyCounterparty = AnyCounterparty;
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

impl CreateClientOptionsParser<HermesApp, CreateClientArgs, Index<0>, Index<1>>
    for HermesAppComponents
{
    async fn parse_create_client_options(
        _app: &HermesApp,
        args: &CreateClientArgs,
        target_chain: &CosmosChain,
        counterparty_chain: &CosmosChain,
    ) -> Result<((), CosmosCreateClientOptions), Error> {
        let max_clock_drift = match args.clock_drift.map(|d| d.into()) {
            Some(input) => input,
            None => {
                target_chain.chain_config.clock_drift
                    + counterparty_chain.chain_config.clock_drift
                    + counterparty_chain.chain_config.max_block_time
            }
        };

        let settings = CosmosCreateClientOptions {
            max_clock_drift,
            trusting_period: args.trusting_period.map(|d| d.into()).unwrap_or_default(),
            trust_threshold: args
                .trust_threshold
                .map(|threshold| threshold.into())
                .unwrap_or_default(),
        };

        Ok(((), settings))
    }
}

pub trait CanUseHermesApp:
    CanLoadConfig
    + CanWriteConfig
    + CanLoadBuilder
    + CanRunCommand<StartRelayerArgs>
    + CanRunCommand<QueryClientSubCommand>
    + CanRunCommand<QueryClientStateArgs>
    + CanRunCommand<QueryConsensusStateArgs>
    + CanRunCommand<QueryClientStatusArgs>
    + CanRunCommand<CreateClientArgs>
    + CanRunCommand<BootstrapChainArgs>
    + CanProduceOutput<&'static str>
    + CanProduceOutput<ClientId>
    + CanRaiseError<HermesError>
    + CanRaiseError<WrapError<&'static str, HermesError>>
{
}

impl CanUseHermesApp for HermesApp {}
