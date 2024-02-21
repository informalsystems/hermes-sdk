use cgp_core::CanRaiseError;

use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;
use hermes_cosmos_client_components::traits::chain_handle::HasBlockingChainHandle;
use hermes_cosmos_relayer::contexts::{builder::CosmosBuilder, chain::CosmosChain};

use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer::chain::requests::QueryChannelClientStateRequest;
use ibc_relayer_types::core::ics24_host::identifier::{ChainId, ChannelId, PortId};
use oneline_eyre::eyre::Context;

use crate::Result;

#[derive(Debug, clap::Parser)]
pub struct QueryChannelClient {
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
}

impl CommandRunner<CosmosBuilder> for QueryChannelClient {
    async fn run(&self, builder: &CosmosBuilder) -> Result<Output> {
        let channel_id = self.channel_id.clone();
        let port_id = self.port_id.clone();
        let chain = builder
            .build_chain(&self.chain_id)
            .await
            .wrap_err_with(|| format!("failed to build chain `{}`", self.chain_id))?;

        let client_state = chain
            .with_blocking_chain_handle(move |chain_handle| {
                chain_handle
                    .query_channel_client_state(QueryChannelClientStateRequest {
                        port_id,
                        channel_id,
                    })
                    .map_err(|e| {
                        CosmosChain::raise_error(
                            e.1.wrap_err("failed to query client state for channel"),
                        )
                    })
            })
            .await
            .wrap_err("`query channel client` command failed")?;

        Ok(Output::success(client_state))
    }
}
