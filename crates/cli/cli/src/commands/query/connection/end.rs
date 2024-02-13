use hermes_cli_framework::output::Output;
use oneline_eyre::eyre::eyre;
use tracing::error;
use tracing::info;

use hermes_cli_framework::command::CommandRunner;
use hermes_cosmos_client_components::traits::chain_handle::HasBlockingChainHandle;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;
use hermes_cosmos_relayer::types::error::BaseError as RelayerError;
use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer::chain::requests::QueryConnectionRequest;
use ibc_relayer::chain::requests::{IncludeProof, QueryHeight};
use ibc_relayer_types::core::ics03_connection::connection::State;
use ibc_relayer_types::core::ics24_host::identifier::ChainId;
use ibc_relayer_types::core::ics24_host::identifier::ConnectionId;
use ibc_relayer_types::Height;

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
        let chain_id = self.chain_id.clone();
        let connection_id = self.connection_id.clone();
        let height = self.height;

        let connection_end = chain
            .with_blocking_chain_handle(move |chain_handle| {
                let query_height = if let Some(height) = height {
                    let specified_height = Height::new(chain_handle.id().version(), height)
                    .map_err(|e| RelayerError::generic(eyre!("Failed to create Height with revision number `{}` and revision height `{height}`. Error: {e}", chain_handle.id().version())))?;

                    QueryHeight::Specific(specified_height)
                } else {
                    QueryHeight::Latest
                };
                let (connection_end, _) = chain_handle
                    .query_connection(
                        QueryConnectionRequest {
                            connection_id: connection_id.clone(),
                            height: query_height,
                        },
                        IncludeProof::No,
                    )
                    .map_err(|e| RelayerError::generic(eyre!("Failed to query connection with id `{connection_id}`. Error: {e}")))?;

                if connection_end.state_matches(&State::Uninitialized) {
                    error!("Connection '{connection_id}' does not exist")
                } else {
                    info!(
                        "Successfully queried connection end on chain `{}`",
                        chain_id,
                    );
                }

                Ok(connection_end)
            })
            .await?;

        Ok(Output::success(connection_end))
    }
}
