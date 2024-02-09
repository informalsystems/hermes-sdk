use hermes_cli_framework::command::Runnable;
use hermes_cli_framework::output::Output;
use hermes_cosmos_client_components::traits::chain_handle::HasBlockingChainHandle;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;
use hermes_cosmos_relayer::types::error::BaseError;
use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer::chain::requests::{IncludeProof, QueryClientStateRequest, QueryHeight};
use ibc_relayer_types::core::ics02_client::height::Height;
use ibc_relayer_types::core::ics24_host::identifier::{ChainId, ClientId};
use oneline_eyre::eyre::Context;
use tracing::info;

use crate::Result;

#[derive(Debug, clap::Parser)]
pub struct QueryClientState {
    /// Identifier of the host chain
    #[clap(
        long = "chain",
        required = true,
        value_name = "CHAIN_ID",
        help_heading = "REQUIRED"
    )]
    chain_id: ChainId,

    /// Identifier of the client on the host chain
    #[clap(
        long = "client",
        required = true,
        value_name = "CLIENT_ID",
        help_heading = "REQUIRED"
    )]
    client_id: ClientId,

    #[clap(
        long = "height",
        value_name = "HEIGHT",
        help = "The height at which to query the client state. If not specified, the latest height is used."
    )]
    height: Option<u64>,
}

impl Runnable for QueryClientState {
    async fn run(&self, builder: CosmosBuilder) -> Result<Output> {
        let chain = builder.build_chain(&self.chain_id).await?;

        let height = self.height.map_or(QueryHeight::Latest, |height| {
            QueryHeight::Specific(Height::new(self.chain_id.version(), height).unwrap())
        });

        let client_id = self.client_id.clone();

        let client_state = chain
            .with_blocking_chain_handle(move |handle| {
                let (client_state, _) = handle
                    .query_client_state(
                        QueryClientStateRequest { client_id, height },
                        IncludeProof::No,
                    )
                    .map_err(BaseError::relayer)?;

                Ok(client_state)
            })
            .await
            .wrap_err_with(|| {
                format!(
                    "Failed to query client state for client `{}` on chain `{}`",
                    self.client_id, self.chain_id
                )
            })?;

        info!(
            "Found client state for client `{}` on chain `{}`:",
            self.client_id, self.chain_id
        );

        Ok(Output::success(client_state))
    }
}
