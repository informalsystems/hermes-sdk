use oneline_eyre::eyre::{eyre, Context};

use cgp_core::CanRaiseError;

use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;
use hermes_cosmos_client_components::traits::chain_handle::HasBlockingChainHandle;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;

use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer::chain::requests::{IncludeProof, QueryChannelRequest, QueryHeight};
use ibc_relayer_types::core::ics04_channel::channel::State;
use ibc_relayer_types::core::ics24_host::identifier::{ChainId, ChannelId, PortId};
use ibc_relayer_types::Height;

use crate::Result;

#[derive(Debug, clap::Parser)]
pub struct QueryChannelEnd {
    #[clap(
        long = "chain",
        required = true,
        value_name = "CHAIN_ID",
        help_heading = "REQUIRED",
        help = "Identifier of the chain to query"
    )]
    chain_id: ChainId,

    #[clap(
        long = "port",
        required = true,
        value_name = "PORT_ID",
        help_heading = "REQUIRED",
        help = "Identifier of the port to query"
    )]
    port_id: PortId,

    #[clap(
        long = "channel",
        required = true,
        value_name = "CHANNEL_ID",
        help_heading = "REQUIRED",
        help = "Identifier of the channel to query"
    )]
    channel_id: ChannelId,

    #[clap(
        long = "height",
        value_name = "HEIGHT",
        help = "Height of the state to query. Leave unspecified for the latest height."
    )]
    height: Option<u64>,
}

impl CommandRunner<CosmosBuilder> for QueryChannelEnd {
    async fn run(&self, builder: &CosmosBuilder) -> Result<Output> {
        let channel_id = self.channel_id.clone();
        let port_id = self.port_id.clone();
        let height = self.height;
        let chain = builder
            .build_chain(&self.chain_id)
            .await
            .wrap_err_with(|| format!("failed to build chain `{}`", self.chain_id))?;

        let query_height = if let Some(height) = height {
            let specified_height = Height::new(chain.chain_id.version(), height)
                .map_err(|e| CosmosChain::raise_error(e.1.wrap_err(format!("failed to create Height with revision number `{}` and revision height `{height}`", chain.chain_id.version()))))?;

            QueryHeight::Specific(specified_height)
        } else {
            QueryHeight::Latest
        };

        let channel_end = chain
            .with_blocking_chain_handle(move |chain_handle| {
                match chain_handle.query_channel(
                    QueryChannelRequest {
                        port_id,
                        channel_id,
                        height: query_height,
                    },
                    IncludeProof::No,
                ) {
                    Ok((channel_end, _)) => Ok(channel_end),
                    Err(e) => Err(CosmosChain::raise_error(
                        e.1.wrap_err("failed to query channel"),
                    )),
                }
            })
            .await
            .wrap_err("`query channel end` command failed")?;

        if channel_end.state_matches(&State::Uninitialized) {
            Err(eyre!(
                "port `{}` & channel `{}` do not exist",
                self.port_id,
                self.channel_id,
            ))
        } else {
            Ok(Output::success(channel_end))
        }
    }
}
