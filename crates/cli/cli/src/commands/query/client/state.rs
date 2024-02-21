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
use hermes_relayer_components::chain::traits::queries::client_state::{
    CanQueryClientState, CanQueryClientStateWithLatestHeight,
};
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;

use ibc_relayer_types::core::ics02_client::height::Height;
use ibc_relayer_types::core::ics24_host::identifier::{ChainId, ClientId};

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
    ChainA: HasIbcChainTypes<ChainB, ChainId = ChainId, ClientId = ClientId, Height = Height>
        + CanQueryClientState<ChainB>
        + CanQueryClientStateWithLatestHeight<ChainB>,
    ChainB: HasIbcChainTypes<ChainA> + HasClientStateType<ChainA>,
    ChainA::Error: From<BaseError> + StdError,
    Build::Error: From<BaseError> + StdError,
    ChainB::ClientState: Debug + Serialize,
{
    async fn run(&self, builder: &Build) -> Result<Output> {
        let chain_id = &self.chain_id;
        let client_id = &self.client_id;

        let chain = builder
            .build_chain(ChainATarget, &self.chain_id)
            .await
            .wrap_err_with(|| format!("failed to build chain `{}`", self.chain_id))?;

        let client_state = match self.height {
            Some(height) => {
                let height = Height::new(self.chain_id.version(), height).unwrap();
                chain
                    .query_client_state(&self.client_id, &height)
                    .await
                    .wrap_err_with(|| format!("failed to query client `{}`", self.client_id))?
            }
            None => chain
                .query_client_state_with_latest_height(&self.client_id)
                .await
                .wrap_err_with(|| format!("failed to query client `{}`", self.client_id))?,
        };

        info!("Found client state for client `{client_id}` on chain `{chain_id}`!");

        Ok(Output::success(client_state))
    }
}
