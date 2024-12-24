use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_relayer_components::build::traits::builders::chain_builder::CanBuildChain;
use hermes_relayer_components::chain::traits::queries::chain_status::CanQueryChainHeight;
use hermes_relayer_components::chain::traits::queries::channel_end::CanQueryChannelEnd;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::ibc::{HasChannelIdType, HasPortIdType};
use hermes_relayer_components::multi::traits::chain_at::HasChainTypeAt;
use hermes_relayer_components::multi::types::index::Index;

use crate::traits::build::CanLoadBuilder;
use crate::traits::command::CommandRunner;
use crate::traits::output::{CanProduceOutput, HasOutputType};
use crate::traits::parse::CanParseArg;

pub struct RunQueryChannelEndCommand;

#[derive(Debug, clap::Parser, HasField)]
pub struct QueryChannelEndArgs {
    #[clap(
        long = "chain",
        required = true,
        value_name = "CHAIN_ID",
        help_heading = "REQUIRED",
        help = "Identifier of the chain to query"
    )]
    chain_id: String,

    #[clap(
        long = "port",
        required = true,
        value_name = "PORT_ID",
        help_heading = "REQUIRED",
        help = "Identifier of the port to query"
    )]
    port_id: String,

    #[clap(
        long = "channel",
        required = true,
        value_name = "CHANNEL_ID",
        help_heading = "REQUIRED",
        help = "Identifier of the channel to query"
    )]
    channel_id: String,

    #[clap(
        long = "height",
        value_name = "HEIGHT",
        help = "Height of the state to query. Leave unspecified for the latest height."
    )]
    height: Option<String>,
}

impl<App, Args, Build, Chain, Counterparty> CommandRunner<App, Args> for RunQueryChannelEndCommand
where
    App: CanLoadBuilder<Builder = Build>
        + HasOutputType
        + CanProduceOutput<Chain::ChannelEnd>
        + CanParseArg<Args, symbol!("chain_id"), Parsed = Chain::ChainId>
        + CanParseArg<Args, symbol!("port_id"), Parsed = Chain::PortId>
        + CanParseArg<Args, symbol!("channel_id"), Parsed = Chain::ChannelId>
        + CanParseArg<Args, symbol!("height"), Parsed = Option<Chain::Height>>
        + CanRaiseError<Build::Error>
        + CanRaiseError<Chain::Error>,
    Build: CanBuildChain<Index<0>, Chain = Chain> + HasChainTypeAt<Index<1>, Chain = Counterparty>,
    Chain: HasChainIdType
        + HasPortIdType<Counterparty>
        + HasChannelIdType<Counterparty>
        + HasHeightType
        + CanQueryChannelEnd<Counterparty>
        + CanQueryChainHeight,
    Args: Async,
{
    async fn run_command(app: &App, args: &Args) -> Result<App::Output, App::Error> {
        let builder = app.load_builder().await?;

        let chain_id = app.parse_arg(args, PhantomData::<symbol!("chain_id")>)?;
        let port_id = app.parse_arg(args, PhantomData::<symbol!("port_id")>)?;
        let channel_id = app.parse_arg(args, PhantomData::<symbol!("channel_id")>)?;
        let height = app.parse_arg(args, PhantomData::<symbol!("height")>)?;

        let chain = builder
            .build_chain(PhantomData::<Index<0>>, &chain_id)
            .await
            .map_err(App::raise_error)?;

        let query_height = match height {
            Some(height) => height,
            None => chain.query_chain_height().await.map_err(App::raise_error)?,
        };

        let channel_end = chain
            .query_channel_end(&channel_id, &port_id, &query_height)
            .await
            .map_err(App::raise_error)?;

        Ok(app.produce_output(channel_end))
    }
}
