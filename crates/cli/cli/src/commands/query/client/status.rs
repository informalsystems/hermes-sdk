use std::error::Error as StdError;
use std::fmt::Debug;

use oneline_eyre::eyre::Context;
use serde::Serialize;
use tracing::info;

use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;
use hermes_cosmos_relayer::types::error::BaseError;
use hermes_relayer_components::birelay::traits::two_way::HasTwoWayRelayTypes;
use hermes_relayer_components::build::traits::components::chain_builder::CanBuildChain;
use hermes_relayer_components::build::traits::target::chain::ChainATarget;
use hermes_relayer_components::chain::traits::queries::client_status::{
    CanQueryClientStatus, ClientStatus,
};
use hermes_relayer_components::chain::traits::types::client_state::{
    HasClientStateFields, HasClientStateType,
};
use hermes_relayer_components::chain::traits::types::consensus_state::{
    HasConsensusStateFields, HasConsensusStateType,
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
        + CanQueryClientStatus<Counterparty>,
    Counterparty: HasIbcChainTypes<Chain>
        + HasClientStateType<Chain>
        + HasClientStateFields<Chain>
        + HasConsensusStateType<Chain>
        + HasConsensusStateFields<Chain>,
    Chain::Error: From<BaseError> + StdError,
    Build::Error: From<BaseError> + StdError,
{
    async fn run(&self, builder: &Build) -> Result<Output> {
        let chain = builder
            .build_chain(ChainATarget, &self.chain_id)
            .await
            .wrap_err_with(|| format!("failed to build chain `{}`", self.chain_id))?;

        let client_status = chain
            .query_client_status(&self.client_id)
            .await
            .wrap_err_with(|| format!("failed to query client `{}`", self.client_id))?;

        match client_status {
            ClientStatus::Frozen => {
                info!("Client `{}` is frozen", self.client_id);
            }
            ClientStatus::Expired => {
                info!("Client `{}` has expired", self.client_id);
            }
            ClientStatus::Active => {
                info!("Client `{}` is active", self.client_id);
            }
        }

        Ok(Output::success(Status::from(client_status)))
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
enum Status {
    Frozen,
    Expired,
    Active,
}

impl From<ClientStatus> for Status {
    fn from(status: ClientStatus) -> Self {
        match status {
            ClientStatus::Frozen => Status::Frozen,
            ClientStatus::Expired => Status::Expired,
            ClientStatus::Active => Status::Active,
        }
    }
}
