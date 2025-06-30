use core::fmt::Display;
use core::marker::PhantomData;

use cgp::core::field::Index;
use hermes_core::logging_components::traits::CanLog;
use hermes_core::logging_components::types::LevelInfo;
use hermes_core::relayer_components::build::traits::builders::chain_builder::CanBuildChain;
use hermes_core::relayer_components::chain::traits::{
    CanMeasureTime, CanQueryChainStatus, CanQueryClientState, CanQueryClientStateWithLatestHeight,
    CanQueryConsensusState, CanQueryConsensusStateWithLatestHeight, HasClientStateFields,
    HasClientStateType, HasConsensusStateFields, HasConsensusStateType, HasIbcChainTypes,
};
use hermes_core::relayer_components::multi::traits::chain_at::HasChainTypeAt;
use hermes_prelude::*;
use serde::Serialize;

use crate::traits::{
    CanLoadBuilder, CanParseArg, CanProduceOutput, CommandRunner, CommandRunnerComponent,
};

pub struct RunQueryClientStatusCommand;

#[derive(Debug, clap::Parser, HasField)]
pub struct QueryClientStatusArgs {
    /// Identifier of the host chain
    #[clap(
        long = "chain",
        required = true,
        value_name = "CHAIN_ID",
        help_heading = "REQUIRED"
    )]
    chain_id: String,

    /// Identifier of the client on the host chain
    #[clap(
        long = "client",
        required = true,
        value_name = "CLIENT_ID",
        help_heading = "REQUIRED"
    )]
    client_id: String,
}

#[derive(Debug, Serialize)]
pub enum ClientStatus {
    Frozen,
    Expired,
    Active,
}

#[cgp_provider(CommandRunnerComponent)]
impl<App, Args, Build, Chain, Counterparty> CommandRunner<App, Args> for RunQueryClientStatusCommand
where
    App: CanLoadBuilder<Builder = Build>
        + CanProduceOutput<ClientStatus>
        + CanLog<LevelInfo>
        // TODO: use AnyCounterparty
        // + HasAnyCounterparty<AnyCounterparty = Counterparty>
        + CanParseArg<Args, symbol!("chain_id"), Parsed = Chain::ChainId>
        + CanParseArg<Args, symbol!("client_id"), Parsed = Chain::ClientId>
        + CanRaiseAsyncError<Build::Error>
        + CanRaiseAsyncError<Chain::Error>,
    Build: CanBuildChain<Index<0>, Chain = Chain> + HasChainTypeAt<Index<1>, Chain = Counterparty>,
    Chain: HasIbcChainTypes<Counterparty> + CanQueryClientStatus<Counterparty>,
    Counterparty: HasIbcChainTypes<Chain>
        + HasClientStateType<Chain>
        + HasClientStateFields<Chain>
        + HasConsensusStateType<Chain>
        + HasConsensusStateFields<Chain>,
    Args: Async,
    Chain::ClientId: Display,
{
    async fn run_command(app: &App, args: &Args) -> Result<App::Output, App::Error> {
        let chain_id = app.parse_arg(args, PhantomData::<symbol!("chain_id")>)?;
        let client_id = app.parse_arg(args, PhantomData::<symbol!("client_id")>)?;

        let builder = app.load_builder().await?;

        let chain = builder
            .build_chain(PhantomData, &chain_id)
            .await
            .map_err(App::raise_error)?;

        let client_status = chain
            .query_client_status(&client_id)
            .await
            .map_err(App::raise_error)?;

        match client_status {
            ClientStatus::Frozen => {
                app.log(&format!("Client `{client_id}` is frozen"), &LevelInfo)
                    .await;
            }
            ClientStatus::Expired => {
                app.log(&format!("Client `{client_id}` has expired"), &LevelInfo)
                    .await;
            }
            ClientStatus::Active => {
                app.log(&format!("Client `{client_id}` is active"), &LevelInfo)
                    .await;
            }
        }

        Ok(app.produce_output(client_status))
    }
}

#[async_trait]
pub trait CanQueryClientStatus<Counterparty>:
    HasIbcChainTypes<Counterparty>
    + HasAsyncErrorType
    + CanQueryClientState<Counterparty>
    + CanQueryClientStateWithLatestHeight<Counterparty>
    + CanQueryChainStatus
    + CanQueryConsensusState<Counterparty>
where
    Counterparty: HasIbcChainTypes<Self>
        + HasClientStateType<Self>
        + HasClientStateFields<Self>
        + HasConsensusStateType<Self>
        + HasConsensusStateFields<Self>,
{
    async fn query_client_status(
        &self,
        client_id: &Self::ClientId,
    ) -> Result<ClientStatus, Self::Error>;
}

impl<Chain, Counterparty> CanQueryClientStatus<Counterparty> for Chain
where
    Chain: HasIbcChainTypes<Counterparty>
        + HasAsyncErrorType
        + CanQueryClientState<Counterparty>
        + CanQueryClientStateWithLatestHeight<Counterparty>
        + CanQueryChainStatus
        + CanQueryConsensusState<Counterparty>
        + CanMeasureTime
        + CanWrapError<String>,
    Counterparty: HasIbcChainTypes<Chain>
        + HasClientStateType<Chain>
        + HasClientStateFields<Chain>
        + HasConsensusStateType<Chain>
        + HasConsensusStateFields<Chain>,
    Chain::ClientId: Display,
{
    async fn query_client_status(
        &self,
        client_id: &Self::ClientId,
    ) -> Result<ClientStatus, Self::Error> {
        let client_state = self
            .query_client_state_with_latest_height(PhantomData, client_id)
            .await
            .map_err(|e| {
                Chain::wrap_error(
                    e,
                    format!("Failed to query client state for client `{client_id}`"),
                )
            })?;

        if Counterparty::client_state_is_frozen(&client_state) {
            return Ok(ClientStatus::Frozen);
        }

        let client_latest_height = Counterparty::client_state_latest_height(&client_state);

        let latest_consensus_state = self
            .query_consensus_state_with_latest_height(PhantomData, client_id, &client_latest_height)
            .await
            .map_err(|e| {
                Chain::wrap_error(
                    e,
                    format!("Failed to query consensus state at height {client_latest_height}"),
                )
            })?;

        let latest_consensus_state_timestamp =
            Counterparty::consensus_state_timestamp(&latest_consensus_state);

        let chain_status = self
            .query_chain_status()
            .await
            .map_err(|e| Chain::wrap_error(e, "Failed to query chain status".to_owned()))?;

        let current_network_time = Self::chain_status_time(&chain_status);

        let elapsed = Self::duration_since(&latest_consensus_state_timestamp, current_network_time);

        let has_expired = elapsed
            .is_some_and(|elapsed| Counterparty::client_state_has_expired(&client_state, elapsed));

        if has_expired {
            Ok(ClientStatus::Expired)
        } else {
            Ok(ClientStatus::Active)
        }
    }
}
