use hermes_cli_components::traits::build::CanLoadBuilder;
use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_relayer_components::chain::traits::queries::chain_status::CanQueryChainHeight;
use hermes_relayer_components::chain::traits::queries::connection_end::CanQueryConnectionEnd;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainId;
use ibc::core::client::types::Height;
use ibc::core::connection::types::State;
use ibc::core::host::types::identifiers::{ChainId, ConnectionId};
use oneline_eyre::eyre::eyre;

use crate::contexts::app::HermesApp;
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

impl CommandRunner<HermesApp> for QueryConnectionEnd {
    async fn run(&self, app: &HermesApp) -> Result<Output> {
        let builder = app.load_builder().await?;

        let chain = builder.build_chain(&self.chain_id).await?;

        let height = match self.height {
            Some(height) => {
                Height::new(chain.chain_id().revision_number(), height)
                    .map_err(|e| eyre!("Failed to create Height with revision number `{}` and revision height `{height}`. Error: {e}", chain.chain_id().revision_number()))?
            }
            None => {
                chain.query_chain_height().await?
            }
        };

        let connection_end =
            <CosmosChain as CanQueryConnectionEnd<CosmosChain>>::query_connection_end(
                &chain,
                &self.connection_id,
                &height,
            )
            .await?;

        if connection_end.state == State::Uninitialized {
            return Ok(Output::error(format!(
                "Connection '{}' does not exist",
                self.connection_id
            )));
        }

        Ok(Output::success(connection_end))
    }
}
