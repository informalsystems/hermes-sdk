use core::fmt::Display;
use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_logging_components::traits::has_logger::HasLogger;
use hermes_logging_components::traits::logger::CanLog;
use hermes_logging_components::types::level::LevelInfo;
use hermes_relayer_components::build::traits::builders::chain_builder::CanBuildChain;
use hermes_relayer_components::chain::traits::queries::client_state::CanQueryAllClientStates;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::multi::traits::chain_at::HasChainTypeAt;
use hermes_relayer_components::multi::types::index::Index;

use crate::traits::build::CanLoadBuilder;
use crate::traits::command::CommandRunner;
use crate::traits::output::{CanProduceOutput, HasOutputType};
use crate::traits::parse::CanParseArg;

pub struct RunQueryClientsCommand;

#[derive(Debug, clap::Parser, HasField)]
pub struct QueryClients {
    #[clap(
        long = "host-chain",
        required = true,
        value_name = "HOST_CHAIN_ID",
        help_heading = "REQUIRED",
        help = "Identifier of the host chain to query"
    )]
    host_chain_id: String,

    #[clap(
        long = "reference-chain",
        value_name = "REFERENCE_CHAIN_ID",
        help = "Only show clients that reference this chain"
    )]
    reference_chain_id: Option<String>,
}

impl<App, Args, Build, Chain, ClientId, Counterparty> CommandRunner<App, Args>
    for RunQueryClientsCommand
where
    App: CanLoadBuilder<Builder = Build>
        + HasOutputType
        + HasLogger
        + CanProduceOutput<Vec<ClientId>>
        + CanParseArg<Args, symbol!("host_chain_id"), Parsed = Chain::ChainId>
        + CanParseArg<Args, symbol!("reference_chain_id"), Parsed = Chain::ChainId>
        + CanRaiseError<Build::Error>
        + CanRaiseError<Chain::Error>,
    Build: CanBuildChain<Index<0>, Chain = Chain> + HasChainTypeAt<Index<1>, Chain = Counterparty>,
    Chain: HasChainIdType + HasErrorType,
    Counterparty: HasClientStateType<Counterparty> + CanQueryAllClientStates<Counterparty>,
    Args: Async,
    App::Logger: CanLog<LevelInfo>,
    ClientId: Display,
{
    async fn run_command(app: &App, args: &Args) -> Result<App::Output, App::Error> {
        let builder = app.load_builder().await?;

        let host_chain_id = app.parse_arg(args, PhantomData::<symbol!("host_chain_id")>)?;
        let reference_chain_id =
            app.parse_arg(args, PhantomData::<symbol!("reference_chain_id")>)?;

        let chain = builder
            .build_chain(PhantomData::<Index<0>>, &host_chain_id)
            .await
            .map_err(App::raise_error)?;

        let clients =
            query_all_client_states::<Chain, Counterparty>(&host_chain_id, &reference_chain_id)
                .await?;

        Ok(app.produce_output(clients))
    }
}
