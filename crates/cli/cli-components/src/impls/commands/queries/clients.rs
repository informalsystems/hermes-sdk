use core::fmt::Display;
use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_logging_components::traits::has_logger::HasLogger;
use hermes_logging_components::traits::logger::CanLog;
use hermes_logging_components::types::level::LevelInfo;
use hermes_relayer_components::build::traits::builders::chain_builder::CanBuildChain;
use hermes_relayer_components::chain::traits::queries::client_state::CanQueryAllClientStatesWithLatestHeight;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateFields;
use hermes_relayer_components::chain::traits::types::ibc::HasClientIdType;
use hermes_relayer_components::multi::traits::chain_at::HasChainTypeAt;
use hermes_relayer_components::multi::types::index::Index;

use crate::traits::build::CanLoadBuilder;
use crate::traits::command::CommandRunner;
use crate::traits::output::{CanProduceOutput, HasOutputType};
use crate::traits::parse::CanParseArg;

pub struct RunQueryClientsCommand;

#[derive(Debug, clap::Parser, HasField)]
pub struct QueryClientsArgs {
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

impl<App, Args, Build, Chain, Counterparty> CommandRunner<App, Args> for RunQueryClientsCommand
where
    App: CanLoadBuilder<Builder = Build>
        + HasLogger
        + HasOutputType
        + CanProduceOutput<Vec<(Chain::ClientId, Counterparty::ClientState)>>
        + CanParseArg<Args, symbol!("host_chain_id"), Parsed = Chain::ChainId>
        + CanParseArg<Args, symbol!("reference_chain_id"), Parsed = Option<Counterparty::ChainId>>
        + CanRaiseError<Build::Error>
        + CanRaiseError<Chain::Error>,
    Build: CanBuildChain<Index<0>, Chain = Chain> + HasChainTypeAt<Index<1>, Chain = Counterparty>,
    Chain: HasChainIdType
        + HasClientIdType<Counterparty>
        + CanQueryAllClientStatesWithLatestHeight<Counterparty>,
    Counterparty: HasClientIdType<Counterparty> + HasClientStateFields<Chain>,
    Args: Async,
    Chain::ClientId: Display,
    App::Logger: CanLog<LevelInfo>,
    Counterparty::ChainId: Eq,
{
    async fn run_command(app: &App, args: &Args) -> Result<App::Output, App::Error> {
        let builder = app.load_builder().await?;
        let logger = app.logger();

        let host_chain_id = app.parse_arg(args, PhantomData::<symbol!("host_chain_id")>)?;
        let reference_chain_id =
            app.parse_arg(args, PhantomData::<symbol!("reference_chain_id")>)?;

        let chain = builder
            .build_chain(PhantomData::<Index<0>>, &host_chain_id)
            .await
            .map_err(App::raise_error)?;

        let all_client_states = chain
            .query_all_client_states_with_latest_height()
            .await
            .map_err(App::raise_error)?
            .into_iter()
            .collect::<Vec<_>>();

        let mut result_client_states = Vec::new();

        for (client_id, client_state) in all_client_states.into_iter() {
            let chain_id = Counterparty::client_state_chain_id(&client_state);

            if let Some(reference_chain_id) = &reference_chain_id {
                if reference_chain_id == &chain_id {
                    continue;
                }
            }

            logger
                .log(
                    &format!("- {}: {} -> {}", client_id, &host_chain_id, chain_id,),
                    &LevelInfo,
                )
                .await;

            result_client_states.push((client_id, client_state));
        }

        Ok(app.produce_output(result_client_states))
    }
}
