use core::fmt::Debug;
use core::time::Duration;
use std::path::PathBuf;

use cgp::core::component::{UseContext, UseDelegate};
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent, ErrorWrapperComponent};
use cgp::core::field::Index;
use cgp::core::types::WithType;
use cgp::prelude::*;
use hermes_any_counterparty::contexts::any_counterparty::AnyCounterparty;
use hermes_cli_components::impls::commands::bootstrap::chain::RunBootstrapChainCommand;
use hermes_cli_components::impls::commands::channel::create::RunCreateChannelCommand;
use hermes_cli_components::impls::commands::client::create::{
    CreateClientOptionsParser, CreateClientOptionsParserComponent, RunCreateClientCommand,
};
use hermes_cli_components::impls::commands::client::update::{
    RunUpdateClientCommand, UpdateClientArgs,
};
use hermes_cli_components::impls::commands::connection::create::{
    CreateConnectionArgs, RunCreateConnectionCommand,
};
use hermes_cli_components::impls::commands::queries::balance::{
    QueryBalanceArgs, RunQueryBalanceCommand,
};
use hermes_cli_components::impls::commands::queries::chain::{
    QueryChainSubCommand, RunQueryChainSubCommand,
};
use hermes_cli_components::impls::commands::queries::chain_status::{
    QueryChainStatusArgs, RunQueryChainStatusCommand,
};
use hermes_cli_components::impls::commands::queries::channel::{
    QueryChannelSubCommand, RunQueryChannelSubCommand,
};
use hermes_cli_components::impls::commands::queries::channel_end::{
    QueryChannelEndArgs, RunQueryChannelEndCommand,
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
use hermes_cli_components::impls::commands::queries::clients::{
    QueryClientsArgs, RunQueryClientsCommand,
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
use hermes_cli_components::impls::commands::queries::wallet::{
    QueryWalletSubCommand, RunQueryWalletSubCommand,
};
use hermes_cli_components::impls::commands::start::{RunStartRelayerCommand, StartRelayerArgs};
use hermes_cli_components::impls::config::get_config_path::GetDefaultConfigField;
use hermes_cli_components::impls::config::load_toml_config::LoadTomlConfig;
use hermes_cli_components::impls::config::save_toml_config::WriteTomlConfig;
use hermes_cli_components::impls::parse::string::{ParseFromOptionalString, ParseFromString};
use hermes_cli_components::traits::any_counterparty::AnyCounterpartyTypeProviderComponent;
use hermes_cli_components::traits::bootstrap::{
    BootstrapLoaderComponent, BootstrapTypeProviderComponent,
};
use hermes_cli_components::traits::build::{
    BuilderLoaderComponent, BuilderTypeComponent, CanLoadBuilder,
};
use hermes_cli_components::traits::command::{CanRunCommand, CommandRunnerComponent};
use hermes_cli_components::traits::config::config_path::ConfigPathGetterComponent;
use hermes_cli_components::traits::config::load_config::{CanLoadConfig, ConfigLoaderComponent};
use hermes_cli_components::traits::config::write_config::{CanWriteConfig, ConfigWriterComponent};
use hermes_cli_components::traits::output::{
    CanProduceOutput, HasOutputType, OutputProducer, OutputProducerComponent, OutputTypeComponent,
};
use hermes_cli_components::traits::parse::ArgParserComponent;
use hermes_cli_components::traits::types::config::ConfigTypeComponent;
use hermes_cli_framework::output::Output;
use hermes_cosmos_chain_components::impls::RelayerConfig;
use hermes_cosmos_chain_components::types::CosmosCreateClientOptions;
use hermes_cosmos_integration_tests::contexts::bootstrap::CosmosBootstrap;
use hermes_cosmos_relayer::contexts::{CosmosBuilder, CosmosChain};
use hermes_cosmos_test_components::chain::types::Denom;
use hermes_error::types::{Error, HermesError};
use hermes_logging_components::traits::logger::LoggerComponent;
use hermes_relayer_components::error::traits::RetryableErrorComponent;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{
    RuntimeGetterComponent, RuntimeTypeProviderComponent,
};
use hermes_tracing_logging_components::contexts::logger::TracingLogger;
use ibc::clients::tendermint::types::TrustThreshold;
use ibc::core::client::types::Height;
use ibc::core::host::types::identifiers::{ChainId, ChannelId, ClientId, ConnectionId, PortId};
use serde::Serialize;

use crate::commands::bootstrap::chain::{BootstrapCosmosChainArgs, LoadCosmosBootstrap};
use crate::commands::bootstrap::subcommand::{BootstrapSubCommand, RunBootstrapSubCommand};
use crate::commands::channel::CreateChannelArgs;
use crate::commands::client::create::CreateCosmosClientArgs;
use crate::impls::build::LoadCosmosBuilder;
use crate::impls::error::ProvideCliError;
use crate::impls::parse::ParseInitCosmosChannelOptions;

#[cgp_context(HermesAppComponents)]
#[derive(HasField)]
pub struct HermesApp {
    pub config_path: PathBuf,
    pub runtime: HermesRuntime,
}

pub struct HermesParserComponents;

pub struct HermesCommandRunnerComponents;

delegate_components! {
    HermesAppComponents {
        [
            ErrorTypeProviderComponent,
            ErrorRaiserComponent,
            ErrorWrapperComponent,
            RetryableErrorComponent,
        ]:
            ProvideCliError,
        RuntimeTypeProviderComponent: UseType<HermesRuntime>,
        RuntimeGetterComponent: UseField<symbol!("runtime")>,
        LoggerComponent: TracingLogger,
        AnyCounterpartyTypeProviderComponent:
            UseType<AnyCounterparty>,
        ConfigTypeComponent:
            WithType<RelayerConfig>,
        BootstrapTypeProviderComponent:
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

        (QueryChannelEndArgs, symbol!("chain_id")): ParseFromString<ChainId>,
        (QueryChannelEndArgs, symbol!("port_id")): ParseFromString<PortId>,
        (QueryChannelEndArgs, symbol!("channel_id")): ParseFromString<ChannelId>,
        (QueryChannelEndArgs, symbol!("height")): ParseFromOptionalString<Height>,

        (QueryClientsArgs, symbol!("host_chain_id")): ParseFromString<ChainId>,
        (QueryClientsArgs, symbol!("reference_chain_id")): ParseFromOptionalString<ChainId>,

        (QueryClientStateArgs, symbol!("chain_id")): ParseFromString<ChainId>,
        (QueryClientStateArgs, symbol!("client_id")): ParseFromString<ClientId>,
        (QueryClientStateArgs, symbol!("height")): ParseFromOptionalString<Height>,

        (QueryConsensusStateArgs, symbol!("chain_id")): ParseFromString<ChainId>,
        (QueryConsensusStateArgs, symbol!("client_id")): ParseFromString<ClientId>,
        (QueryConsensusStateArgs, symbol!("query_height")): ParseFromOptionalString<Height>,
        (QueryConsensusStateArgs, symbol!("consensus_height")): ParseFromOptionalString<Height>,

        (QueryChainStatusArgs, symbol!("chain_id")): ParseFromString<ChainId>,

        (QueryBalanceArgs, symbol!("chain_id")): ParseFromString<ChainId>,
        (QueryBalanceArgs, symbol!("address")): ParseFromString<String>,
        (QueryBalanceArgs, symbol!("denom")): ParseFromString<Denom>,

        (StartRelayerArgs, symbol!("chain_id_a")): ParseFromString<ChainId>,
        (StartRelayerArgs, symbol!("client_id_a")): ParseFromString<ClientId>,
        (StartRelayerArgs, symbol!("chain_id_b")): ParseFromString<ChainId>,
        (StartRelayerArgs, symbol!("client_id_b")): ParseFromString<ClientId>,

        (QueryClientStatusArgs, symbol!("chain_id")): ParseFromString<ChainId>,
        (QueryClientStatusArgs, symbol!("client_id")): ParseFromString<ClientId>,

        (CreateCosmosClientArgs, symbol!("target_chain_id")): ParseFromString<ChainId>,
        (CreateCosmosClientArgs, symbol!("counterparty_chain_id")): ParseFromString<ChainId>,

        (CreateChannelArgs, symbol!("target_chain_id")): ParseFromString<ChainId>,
        (CreateChannelArgs, symbol!("target_client_id")): ParseFromString<ClientId>,
        (CreateChannelArgs, symbol!("target_port_id")): ParseFromString<PortId>,
        (CreateChannelArgs, symbol!("counterparty_chain_id")): ParseFromString<ChainId>,
        (CreateChannelArgs, symbol!("counterparty_client_id")): ParseFromString<ClientId>,
        (CreateChannelArgs, symbol!("counterparty_port_id")): ParseFromString<PortId>,
        (CreateChannelArgs, symbol!("init_channel_options")): ParseInitCosmosChannelOptions,

        (CreateConnectionArgs, symbol!("target_chain_id")): ParseFromString<ChainId>,
        (CreateConnectionArgs, symbol!("target_client_id")): ParseFromString<ClientId>,
        (CreateConnectionArgs, symbol!("counterparty_chain_id")): ParseFromString<ChainId>,
        (CreateConnectionArgs, symbol!("counterparty_client_id")): ParseFromString<ClientId>,

        (UpdateClientArgs, symbol!("host_chain_id")): ParseFromString<ChainId>,
        (UpdateClientArgs, symbol!("client_id")): ParseFromString<ClientId>,
        (UpdateClientArgs, symbol!("counterparty_client_id")): ParseFromString<ClientId>,
        (UpdateClientArgs, symbol!("target_height")): ParseFromOptionalString<Height>,
    }
}

delegate_components! {
    HermesCommandRunnerComponents {
        StartRelayerArgs: RunStartRelayerCommand<Index<0>, Index<1>>,

        QueryChainSubCommand: RunQueryChainSubCommand,
        QueryChainStatusArgs: RunQueryChainStatusCommand,

        QueryClientSubCommand: RunQueryClientSubCommand,
        QueryClientStateArgs: RunQueryClientStateCommand,
        QueryClientStatusArgs: RunQueryClientStatusCommand,
        QueryConsensusStateArgs: RunQueryConsensusStateCommand,

        CreateCosmosClientArgs: RunCreateClientCommand<Index<0>, Index<1>>,
        CreateConnectionArgs: RunCreateConnectionCommand,
        CreateChannelArgs: RunCreateChannelCommand,
        UpdateClientArgs: RunUpdateClientCommand,

        BootstrapSubCommand: RunBootstrapSubCommand,
        BootstrapCosmosChainArgs: RunBootstrapChainCommand<(), UseContext>,

        QueryClientsArgs: RunQueryClientsCommand,

        QueryConnectionSubCommand: RunQueryConnectionSubCommand,
        QueryConnectionEndArgs: RunQueryConnectionEndCommand,

        QueryChannelSubCommand: RunQueryChannelSubCommand,
        QueryChannelEndArgs: RunQueryChannelEndCommand,

        QueryWalletSubCommand: RunQueryWalletSubCommand,
        QueryBalanceArgs: RunQueryBalanceCommand,
    }
}

#[cgp_provider(OutputProducerComponent)]
impl<App, Value> OutputProducer<App, Value> for HermesAppComponents
where
    App: HasOutputType<Output = Output>,
    Value: Serialize + Debug + Async,
{
    fn produce_output(_app: &App, value: Value) -> Output {
        Output::success(value)
    }
}

#[cgp_provider(CreateClientOptionsParserComponent)]
impl CreateClientOptionsParser<HermesApp, CreateCosmosClientArgs, Index<0>, Index<1>>
    for HermesAppComponents
{
    async fn parse_create_client_options(
        _app: &HermesApp,
        args: &CreateCosmosClientArgs,
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
            trusting_period: args
                .trusting_period
                .map(|d| d.into())
                .unwrap_or_else(|| Duration::from_secs(14 * 24 * 3600)),
            trust_threshold: args
                .trust_threshold
                .unwrap_or(TrustThreshold::TWO_THIRDS)
                .into(),
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
    + CanRunCommand<QueryClientsArgs>
    + CanRunCommand<QueryClientStateArgs>
    + CanRunCommand<QueryConsensusStateArgs>
    + CanRunCommand<QueryClientStatusArgs>
    + CanRunCommand<QueryChainStatusArgs>
    + CanRunCommand<QueryBalanceArgs>
    + CanRunCommand<CreateCosmosClientArgs>
    + CanRunCommand<CreateConnectionArgs>
    // + CanRunCommand<CreateChannelArgs>
    + CanRunCommand<UpdateClientArgs>
    + CanRunCommand<BootstrapCosmosChainArgs>
    + CanRunCommand<QueryChannelSubCommand>
    + CanRunCommand<QueryChannelEndArgs>
    + CanProduceOutput<&'static str>
    + CanProduceOutput<ClientId>
    + CanRaiseAsyncError<HermesError>
{
}

impl CanUseHermesApp for HermesApp {}
