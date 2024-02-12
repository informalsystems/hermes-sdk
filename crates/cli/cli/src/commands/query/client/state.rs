use core::fmt::Debug;
use std::error::Error as StdError;

use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;
use hermes_cosmos_client_components::traits::chain_handle::HasBlockingChainHandle;
use hermes_cosmos_client_components::types::tendermint::TendermintClientState;
use hermes_cosmos_relayer::types::error::BaseError;
use hermes_relayer_components::birelay::traits::two_way::HasTwoWayRelayTypes;
use hermes_relayer_components::build::traits::components::chain_builder::CanBuildChain;
use hermes_relayer_components::build::traits::target::chain::ChainATarget;
use hermes_relayer_components::chain::traits::queries::client_state::CanQueryClientState;
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer::chain::requests::{IncludeProof, QueryClientStateRequest, QueryHeight};
use ibc_relayer_types::core::ics02_client::height::Height;
use ibc_relayer_types::core::ics24_host::identifier::{ChainId, ClientId};
use oneline_eyre::eyre::Context;
use serde::Serialize;
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

impl<Build, ChainA, ChainB> CommandRunner<Build> for QueryClientState
where
    Build: CanBuildChain<ChainATarget>,
    Build::BiRelay: HasTwoWayRelayTypes<ChainA = ChainA, ChainB = ChainB>,
    ChainA: HasIbcChainTypes<ChainB, ChainId = ChainId, ClientId = ClientId>
        + CanQueryClientState<ChainB>,
    ChainB: HasIbcChainTypes<ChainA, ChainId = ChainId, ClientId = ClientId>
        + HasClientStateType<ChainA>,
    ChainA::Error: From<BaseError> + StdError,
    Build::Error: From<BaseError> + StdError,
    ChainB::ClientState: Serialize,
{
    async fn run(&self, builder: &Build) -> Result<Output> {
        let chain_id = &self.chain_id;
        let client_id = &self.client_id;

        let chain = builder.build_chain(ChainATarget, &self.chain_id).await?;

        let height = self.height.map_or(QueryHeight::Latest, |height| {
            QueryHeight::Specific(Height::new(self.chain_id.version(), height).unwrap())
        });

        let client_state = chain.query_client_state(&self.client_id).await?;

        info!("Found client state for client `{client_id}` on chain `{chain_id}`!");

        Ok(Output::success(client_state))
    }
}

async fn do_query_client_state<Chain, Counterparty>(
    chain: Chain,
    chain_id: &ChainId,
    client_id: &ClientId,
    height: QueryHeight,
) -> Result<Output>
where
    Chain: HasIbcChainTypes<Counterparty, ClientId = ClientId> + HasBlockingChainHandle,
    Counterparty: HasClientStateType<Chain, ClientState = TendermintClientState>,
    Chain::Error: From<BaseError> + StdError,
{
    let client_state = {
        let client_id = client_id.clone();
        chain.with_blocking_chain_handle(move |handle| {
            let (client_state, _) = handle
                .query_client_state(
                    QueryClientStateRequest { client_id, height },
                    IncludeProof::No,
                )
                .map_err(BaseError::relayer)?;

            Ok(client_state)
        })
    }
    .await
    .wrap_err_with(|| {
        format!("Failed to query client state for client `{client_id}` on chain `{chain_id}`")
    })?;

    info!("Found client state for client `{client_id}` on chain `{chain_id}`!");

    Ok(Output::success(client_state))
}
