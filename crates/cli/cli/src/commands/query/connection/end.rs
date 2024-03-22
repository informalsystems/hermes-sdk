use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_relayer_components::chain::traits::queries::connection_end::CanQueryConnectionEnd;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainId;
use ibc_relayer_types::core::ics03_connection::connection::State;
use ibc_relayer_types::core::ics24_host::identifier::{ChainId, ConnectionId};
use ibc_relayer_types::Height;
use oneline_eyre::eyre::eyre;

use crate::Result;

#[derive(Debug, clap::Parser)]
pub struct QueryConnectionEnd {
    #[clap(
        long = "chain",
        required = true,
        value_name = "CHAIN_ID",
        help_heading = "REQUIRED",
        help = "Identifier of the chain to query"
    )]
    chain_id: ChainId,

    #[clap(
        long = "connection",
        visible_alias = "conn",
        required = true,
        value_name = "CONNECTION_ID",
        help_heading = "REQUIRED",
        help = "Identifier of the connection to query"
    )]
    connection_id: ConnectionId,

    #[clap(
        long = "height",
        value_name = "HEIGHT",
        help = "Height of the state to query. Leave unspecified for latest height."
    )]
    height: Option<u64>,
}

impl CommandRunner<CosmosBuilder> for QueryConnectionEnd {
    async fn run(&self, builder: &CosmosBuilder) -> Result<Output> {
        let chain = builder.build_chain(&self.chain_id).await?;
        let height = self.height.map(|h|
            Height::new(chain.chain_id().version(), h)
                .map_err(|e| eyre!("Failed to create Height with revision number `{}` and revision height `{h}`. Error: {e}", chain.chain_id().version()))
        ).transpose()?;

        let connection_end =
            <CosmosChain as CanQueryConnectionEnd<CosmosChain>>::query_connection_end(
                &chain,
                &self.connection_id,
                height.as_ref(),
            )
            .await?;

        if connection_end.state_matches(&State::Uninitialized) {
            return Ok(Output::error(format!(
                "Connection '{}' does not exist",
                self.connection_id
            )));
        }

        Ok(Output::success(connection_end))
    }
}
