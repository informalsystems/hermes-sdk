use std::error::Error as StdError;
use std::fmt::Debug;

use hermes_relayer_components::chain::traits::types::consensus_state::{
    HasConsensusStateFields, HasConsensusStateType,
};
use oneline_eyre::eyre::Context;
use serde::Serialize;

use hermes_relayer_components::chain::traits::queries::chain_status::CanQueryChainStatus;
use hermes_relayer_components::chain::traits::queries::consensus_state::CanQueryConsensusState;

use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;
use hermes_cosmos_relayer::types::error::BaseError;
use hermes_relayer_components::birelay::traits::two_way::HasTwoWayRelayTypes;
use hermes_relayer_components::build::traits::components::chain_builder::CanBuildChain;
use hermes_relayer_components::build::traits::target::chain::ChainATarget;
use hermes_relayer_components::chain::traits::queries::client_state::{
    CanQueryClientState, CanQueryClientStateWithHeight,
};
use hermes_relayer_components::chain::traits::types::client_state::{
    HasClientStateFields, HasClientStateType,
};
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_relayer_types::core::ics02_client::height::Height;
use ibc_relayer_types::core::ics24_host::identifier::{ChainId, ClientId};

use crate::Result;

#[derive(Debug, clap::Parser)]
pub struct QueryClientStatus {
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
}

impl<Build, Chain, Counterparty> CommandRunner<Build> for QueryClientStatus
where
    Build: CanBuildChain<ChainATarget>,
    Build::BiRelay: HasTwoWayRelayTypes<ChainA = Chain, ChainB = Counterparty>,
    Chain: HasIbcChainTypes<Counterparty, ChainId = ChainId, ClientId = ClientId, Height = Height>
        + CanQueryClientState<Counterparty>
        + CanQueryClientStateWithHeight<Counterparty>
        + CanQueryChainStatus
        + CanQueryConsensusState<Counterparty>,
    Counterparty: HasIbcChainTypes<Chain>
        + HasClientStateType<Chain>
        + HasClientStateFields<Chain>
        + HasConsensusStateType<Chain>
        + HasConsensusStateFields<Chain>,
    Chain::Error: From<BaseError> + StdError,
    Build::Error: From<BaseError> + StdError,
    Counterparty::ClientState: Serialize,
{
    async fn run(&self, builder: &Build) -> Result<Output> {
        let chain = builder.build_chain(ChainATarget, &self.chain_id).await?;
        let client_status = query_client_status(&chain, &self.client_id).await?;

        Ok(Output::success(client_status))
    }
}

#[derive(Debug, Serialize)]
enum Status {
    Frozen,
    Expired,
    Active,
}

async fn query_client_status<Chain, Counterparty>(
    chain: &Chain,
    client_id: &ClientId,
) -> Result<Status>
where
    Chain: HasIbcChainTypes<Counterparty, ChainId = ChainId, ClientId = ClientId, Height = Height>
        + CanQueryClientState<Counterparty>
        + CanQueryClientStateWithHeight<Counterparty>
        + CanQueryChainStatus
        + CanQueryConsensusState<Counterparty>,
    Counterparty: HasIbcChainTypes<Chain>
        + HasClientStateType<Chain>
        + HasClientStateFields<Chain>
        + HasConsensusStateType<Chain>
        + HasConsensusStateFields<Chain>,
    Chain::Error: From<BaseError> + StdError,
{
    let client_state = chain
        .query_client_state(client_id)
        .await
        .wrap_err_with(|| "Failed to query client state for client `{client_id}`")?;

    if Counterparty::client_state_is_frozen(&client_state) {
        return Ok(Status::Frozen);
    }

    let client_latest_height = Counterparty::client_state_latest_height(&client_state);

    let latest_consensus_state = chain
        .query_consensus_state(client_id, client_latest_height)
        .await
        .wrap_err_with(|| {
            format!("Failed to query consensus state at height {client_latest_height}")
        })?;

    let latest_consensus_state_timestamp =
        Counterparty::consensus_state_timestamp(&latest_consensus_state);

    let chain_status = chain
        .query_chain_status()
        .await
        .wrap_err("Failed to query chain status")?;

    let current_network_time = Chain::chain_status_timestamp(&chain_status);

    let elapsed = Chain::timestamp_duration_since(
        latest_consensus_state_timestamp.as_ref(),
        current_network_time,
    );

    let has_expired = elapsed.map_or(false, |elapsed| {
        Counterparty::client_state_has_expired(&client_state, elapsed)
    });

    if has_expired {
        Ok(Status::Expired)
    } else {
        Ok(Status::Active)
    }
}
