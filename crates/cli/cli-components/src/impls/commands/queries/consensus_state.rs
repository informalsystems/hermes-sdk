use core::fmt::Display;
use core::marker::PhantomData;

use cgp::core::field::Index;
use cgp::prelude::*;
use hermes_core::relayer_components::build::traits::builders::chain_builder::CanBuildChain;
use hermes_core::relayer_components::chain::traits::{
    CanQueryChainHeight, CanQueryConsensusState, CanQueryConsensusStateHeights, HasChainIdType,
    HasConsensusStateType, HasHeightType,
};
use hermes_core::relayer_components::multi::traits::chain_at::HasChainTypeAt;

use crate::traits::{
    CanLoadBuilder, CanParseArg, CanProduceOutput, CommandRunner, CommandRunnerComponent,
};

pub struct RunQueryConsensusStateCommand;

#[derive(Debug, clap::Parser, HasField)]
pub struct QueryConsensusStateArgs {
    #[clap(
        long = "chain",
        required = true,
        value_name = "CHAIN_ID",
        help_heading = "REQUIRED",
        help = "Identifier of the chain to query"
    )]
    chain_id: String,

    #[clap(
        long = "client",
        required = true,
        value_name = "CLIENT_ID",
        help_heading = "REQUIRED",
        help = "Identifier of the client to query"
    )]
    client_id: String,

    #[clap(
        long = "consensus-height",
        value_name = "CONSENSUS_HEIGHT",
        help = "Height of the client's consensus state to query, if not specified the latest consensus height is used"
    )]
    consensus_height: Option<String>,

    #[clap(
        long = "query_height",
        value_name = "QUERY_HEIGHT",
        help = "The height at which to query the chain"
    )]
    query_height: Option<String>,
}

#[cgp_provider(CommandRunnerComponent)]
impl<App, Args, Build, Chain, Counterparty> CommandRunner<App, Args>
    for RunQueryConsensusStateCommand
where
    App: CanLoadBuilder<Builder = Build>
        + CanProduceOutput<Counterparty::ConsensusState>
        + CanParseArg<Args, symbol!("chain_id"), Parsed = Chain::ChainId>
        + CanParseArg<Args, symbol!("client_id"), Parsed = Chain::ClientId>
        + CanParseArg<Args, symbol!("query_height"), Parsed = Option<Chain::Height>>
        + CanParseArg<Args, symbol!("consensus_height"), Parsed = Option<Counterparty::Height>>
        + CanRaiseAsyncError<Build::Error>
        + CanRaiseAsyncError<Chain::Error>
        + CanRaiseAsyncError<String>,
    Build: CanBuildChain<Index<0>, Chain = Chain> + HasChainTypeAt<Index<1>, Chain = Counterparty>,
    Chain: CanQueryChainHeight
        + HasChainIdType
        + CanQueryConsensusState<Counterparty>
        + CanQueryConsensusStateHeights<Counterparty>,
    Counterparty: HasHeightType + HasConsensusStateType<Chain>,
    Args: Async,
    Chain::ClientId: Display,
{
    async fn run_command(app: &App, args: &Args) -> Result<App::Output, App::Error> {
        let chain_id = app.parse_arg(args, PhantomData::<symbol!("chain_id")>)?;
        let client_id = app.parse_arg(args, PhantomData::<symbol!("client_id")>)?;
        let m_query_height = app.parse_arg(args, PhantomData::<symbol!("query_height")>)?;
        let m_consensus_height = app.parse_arg(args, PhantomData::<symbol!("consensus_height")>)?;

        let builder = app.load_builder().await?;

        let chain = builder
            .build_chain(PhantomData, &chain_id)
            .await
            .map_err(App::raise_error)?;

        let query_height = match m_query_height {
            Some(query_height) => query_height,
            None => chain.query_chain_height().await.map_err(App::raise_error)?,
        };

        let consensus_height = match m_consensus_height {
            Some(consensus_height) => consensus_height,
            None => {
                let heights = chain
                    .query_consensus_state_heights(&client_id)
                    .await
                    .map_err(App::raise_error)?;

                heights.into_iter().next().ok_or_else(|| {
                    App::raise_error(format!(
                        "no consensus state found for client {} on chain {}",
                        client_id, chain_id
                    ))
                })?
            }
        };

        let consensus_state = chain
            .query_consensus_state(PhantomData, &client_id, &consensus_height, &query_height)
            .await
            .map_err(App::raise_error)?;

        Ok(app.produce_output(consensus_state))
    }
}
