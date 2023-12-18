use cgp_core::{Async, ProvideErrorType};
use cosmos_client_components::types::channel::CosmosInitChannelOptions;
use cosmos_client_components::types::connection::CosmosInitConnectionOptions;
use cosmos_client_components::types::payloads::channel::{
    CosmosChannelOpenAckPayload, CosmosChannelOpenConfirmPayload, CosmosChannelOpenTryPayload,
};
use cosmos_client_components::types::payloads::client::{
    CosmosCreateClientPayload, CosmosUpdateClientPayload,
};
use cosmos_client_components::types::payloads::connection::{
    CosmosConnectionOpenAckPayload, CosmosConnectionOpenConfirmPayload,
    CosmosConnectionOpenInitPayload, CosmosConnectionOpenTryPayload,
};
use cosmos_client_components::types::payloads::packet::{
    CosmosAckPacketPayload, CosmosReceivePacketPayload, CosmosTimeoutUnorderedPacketPayload,
};
use cosmos_client_components::types::tendermint::{
    TendermintClientState, TendermintConsensusState,
};
use ibc_relayer::chain::client::ClientSettings;
use ibc_relayer_components::chain::traits::types::channel::{
    HasChannelHandshakePayloads, HasInitChannelOptionsType,
};
use ibc_relayer_components::chain::traits::types::client_state::HasClientStateType;
use ibc_relayer_components::chain::traits::types::connection::{
    HasConnectionHandshakePayloads, HasInitConnectionOptionsType,
};
use ibc_relayer_components::chain::traits::types::consensus_state::HasConsensusStateType;
use ibc_relayer_components::chain::traits::types::create_client::{
    HasCreateClientOptions, HasCreateClientPayload,
};
use ibc_relayer_components::chain::traits::types::packets::ack::HasAckPacketPayload;
use ibc_relayer_components::chain::traits::types::packets::receive::HasReceivePacketPayload;
use ibc_relayer_components::chain::traits::types::packets::timeout::HasTimeoutUnorderedPacketPayload;
use ibc_relayer_components::chain::traits::types::update_client::HasUpdateClientPayload;
use ibc_relayer_components::runtime::traits::runtime::HasRuntime;
use ibc_relayer_components_extra::telemetry::traits::telemetry::HasTelemetry;
use ibc_relayer_runtime::types::error::TokioRuntimeError;
use ibc_relayer_runtime::types::runtime::TokioRuntimeContext;

use crate::contexts::chain::CosmosChain;
use crate::impls::chain::component::CosmosChainComponents;
use crate::types::error::{BaseError, Error};
use crate::types::telemetry::CosmosTelemetry;

impl<Chain> ProvideErrorType<CosmosChain<Chain>> for CosmosChainComponents
where
    Chain: Async,
{
    type Error = Error;
}

impl<Chain> HasRuntime for CosmosChain<Chain>
where
    Chain: Async,
{
    type Runtime = TokioRuntimeContext;

    fn runtime(&self) -> &TokioRuntimeContext {
        &self.runtime
    }

    fn runtime_error(e: TokioRuntimeError) -> Error {
        BaseError::tokio(e).into()
    }
}

impl<Chain> HasTelemetry for CosmosChain<Chain>
where
    Chain: Async,
{
    type Telemetry = CosmosTelemetry;

    fn telemetry(&self) -> &CosmosTelemetry {
        &self.telemetry
    }
}

impl<Chain, Counterparty> HasClientStateType<Counterparty> for CosmosChain<Chain>
where
    Chain: Async,
{
    type ClientState = TendermintClientState;
}

impl<Chain, Counterparty> HasConsensusStateType<Counterparty> for CosmosChain<Chain>
where
    Chain: Async,
{
    type ConsensusState = TendermintConsensusState;
}

impl<Chain, Counterparty> HasCreateClientOptions<Counterparty> for CosmosChain<Chain>
where
    Chain: Async,
{
    type CreateClientPayloadOptions = ClientSettings;
}

impl<Chain, Counterparty> HasInitConnectionOptionsType<Counterparty> for CosmosChain<Chain>
where
    Chain: Async,
{
    type InitConnectionOptions = CosmosInitConnectionOptions;
}

impl<Chain, Counterparty> HasInitChannelOptionsType<Counterparty> for CosmosChain<Chain>
where
    Chain: Async,
{
    type InitChannelOptions = CosmosInitChannelOptions;
}

impl<Chain, Counterparty> HasCreateClientPayload<Counterparty> for CosmosChain<Chain>
where
    Chain: Async,
{
    type CreateClientPayload = CosmosCreateClientPayload;
}

impl<Chain, Counterparty> HasUpdateClientPayload<Counterparty> for CosmosChain<Chain>
where
    Chain: Async,
{
    type UpdateClientPayload = CosmosUpdateClientPayload;
}

impl<Chain, Counterparty> HasConnectionHandshakePayloads<Counterparty> for CosmosChain<Chain>
where
    Chain: Async,
{
    type ConnectionOpenInitPayload = CosmosConnectionOpenInitPayload;

    type ConnectionOpenTryPayload = CosmosConnectionOpenTryPayload;

    type ConnectionOpenAckPayload = CosmosConnectionOpenAckPayload;

    type ConnectionOpenConfirmPayload = CosmosConnectionOpenConfirmPayload;
}

impl<Chain, Counterparty> HasChannelHandshakePayloads<Counterparty> for CosmosChain<Chain>
where
    Chain: Async,
{
    type ChannelOpenTryPayload = CosmosChannelOpenTryPayload;

    type ChannelOpenAckPayload = CosmosChannelOpenAckPayload;

    type ChannelOpenConfirmPayload = CosmosChannelOpenConfirmPayload;
}

impl<Chain, Counterparty> HasReceivePacketPayload<Counterparty> for CosmosChain<Chain>
where
    Chain: Async,
{
    type ReceivePacketPayload = CosmosReceivePacketPayload;
}

impl<Chain, Counterparty> HasAckPacketPayload<Counterparty> for CosmosChain<Chain>
where
    Chain: Async,
{
    type AckPacketPayload = CosmosAckPacketPayload;
}

impl<Chain, Counterparty> HasTimeoutUnorderedPacketPayload<Counterparty> for CosmosChain<Chain>
where
    Chain: Async,
{
    type TimeoutUnorderedPacketPayload = CosmosTimeoutUnorderedPacketPayload;
}
