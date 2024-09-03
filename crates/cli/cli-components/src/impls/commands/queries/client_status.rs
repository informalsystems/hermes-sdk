use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_error::traits::wrap::CanWrapError;
use hermes_logging_components::traits::has_logger::HasLogger;
use hermes_logging_components::traits::logger::CanLog;
use hermes_logging_components::types::level::LevelInfo;
use hermes_relayer_components::build::traits::builders::chain_builder::CanBuildChain;
use hermes_relayer_components::chain::traits::queries::chain_status::CanQueryChainStatus;
use hermes_relayer_components::chain::traits::queries::client_state::{
    CanQueryClientState, CanQueryClientStateWithLatestHeight,
};
use hermes_relayer_components::chain::traits::queries::consensus_state::{
    CanQueryConsensusState, CanQueryConsensusStateWithLatestHeight,
};
use hermes_relayer_components::chain::traits::types::client_state::{
    HasClientStateFields, HasClientStateType,
};
use hermes_relayer_components::chain::traits::types::consensus_state::{
    HasConsensusStateFields, HasConsensusStateType,
};
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::multi::traits::chain_at::HasChainTypeAt;
use hermes_relayer_components::multi::types::index::Index;
use serde::Serialize;

use crate::traits::build::CanLoadBuilder;
use crate::traits::command::CommandRunner;
use crate::traits::output::CanProduceOutput;
use crate::traits::parse::CanParseArg;

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

impl<App, Args, Build, Chain, Counterparty> CommandRunner<App, Args> for RunQueryClientStatusCommand
where
    App: CanLoadBuilder<Builder = Build>
        + CanProduceOutput<ClientStatus>
        + HasLogger
        // TODO: use AnyCounterparty
        // + HasAnyCounterparty<AnyCounterparty = Counterparty>
        + CanParseArg<Args, symbol!("chain_id"), Parsed = Chain::ChainId>
        + CanParseArg<Args, symbol!("client_id"), Parsed = Chain::ClientId>
        + CanRaiseError<Build::Error>
        + CanRaiseError<Chain::Error>,
    Build: CanBuildChain<0, Chain = Chain> + HasChainTypeAt<1, Chain = Counterparty>,
    Chain: HasIbcChainTypes<Counterparty> + CanQueryClientStatus<Counterparty>,
    Counterparty: HasIbcChainTypes<Chain>
        + HasClientStateType<Chain>
        + HasClientStateFields<Chain>
        + HasConsensusStateType<Chain>
        + HasConsensusStateFields<Chain>,
    App::Logger: CanLog<LevelInfo>,
    Args: Async,
{
    async fn run_command(app: &App, args: &Args) -> Result<App::Output, App::Error> {
        let chain_id = app.parse_arg(args, PhantomData::<symbol!("chain_id")>)?;
        let client_id = app.parse_arg(args, PhantomData::<symbol!("client_id")>)?;

        let logger = app.logger();
        let builder = app.load_builder().await?;

        let chain = builder
            .build_chain(Index::<0>, &chain_id)
            .await
            .map_err(App::raise_error)?;

        let client_status = chain
            .query_client_status(&client_id)
            .await
            .map_err(App::raise_error)?;

        match client_status {
            ClientStatus::Frozen => {
                logger
                    .log(&format!("Client `{}` is frozen", client_id), &LevelInfo)
                    .await;
            }
            ClientStatus::Expired => {
                logger
                    .log(&format!("Client `{}` has expired", client_id), &LevelInfo)
                    .await;
            }
            ClientStatus::Active => {
                logger
                    .log(&format!("Client `{}` is active", client_id), &LevelInfo)
                    .await;
            }
        }

        Ok(app.produce_output(client_status))
    }
}

#[async_trait]
pub trait CanQueryClientStatus<Counterparty>:
    HasIbcChainTypes<Counterparty>
    + HasErrorType
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
        + HasErrorType
        + CanQueryClientState<Counterparty>
        + CanQueryClientStateWithLatestHeight<Counterparty>
        + CanQueryChainStatus
        + CanQueryConsensusState<Counterparty>
        + CanWrapError<String>,
    Counterparty: HasIbcChainTypes<Chain>
        + HasClientStateType<Chain>
        + HasClientStateFields<Chain>
        + HasConsensusStateType<Chain>
        + HasConsensusStateFields<Chain>,
{
    async fn query_client_status(
        &self,
        client_id: &Self::ClientId,
    ) -> Result<ClientStatus, Self::Error> {
        let client_state = self
            .query_client_state_with_latest_height(client_id)
            .await
            .map_err(|e| {
                Chain::wrap_error(
                    format!("Failed to query client state for client `{client_id}`"),
                    e,
                )
            })?;

        if Counterparty::client_state_is_frozen(&client_state) {
            return Ok(ClientStatus::Frozen);
        }

        let client_latest_height = Counterparty::client_state_latest_height(&client_state);

        let latest_consensus_state = self
            .query_consensus_state_with_latest_height(client_id, &client_latest_height)
            .await
            .map_err(|e| {
                Chain::wrap_error(
                    format!("Failed to query consensus state at height {client_latest_height}"),
                    e,
                )
            })?;

        let latest_consensus_state_timestamp =
            Counterparty::consensus_state_timestamp(&latest_consensus_state);

        let chain_status = self
            .query_chain_status()
            .await
            .map_err(|e| Chain::wrap_error("Failed to query chain status".to_owned(), e))?;

        let current_network_time = Self::chain_status_timestamp(&chain_status);

        let elapsed =
            Self::timestamp_duration_since(&latest_consensus_state_timestamp, current_network_time);

        let has_expired = elapsed.map_or(false, |elapsed| {
            Counterparty::client_state_has_expired(&client_state, elapsed)
        });

        if has_expired {
            Ok(ClientStatus::Expired)
        } else {
            Ok(ClientStatus::Active)
        }
    }
}
