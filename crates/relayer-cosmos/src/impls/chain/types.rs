use cgp_core::{Async, HasErrorType};
use ibc_cosmos_client_components::types::channel::CosmosInitChannelOptions;
use ibc_cosmos_client_components::types::connection::CosmosInitConnectionOptions;
use ibc_cosmos_client_components::types::payloads::channel::{
    CosmosChannelOpenAckPayload, CosmosChannelOpenConfirmPayload, CosmosChannelOpenTryPayload,
};
use ibc_cosmos_client_components::types::payloads::client::{
    CosmosCreateClientPayload, CosmosUpdateClientPayload,
};
use ibc_cosmos_client_components::types::payloads::connection::{
    CosmosConnectionOpenAckPayload, CosmosConnectionOpenConfirmPayload,
    CosmosConnectionOpenInitPayload, CosmosConnectionOpenTryPayload,
};
use ibc_cosmos_client_components::types::payloads::packet::{
    CosmosAckPacketPayload, CosmosReceivePacketPayload, CosmosTimeoutUnorderedPacketPayload,
};
use ibc_cosmos_client_components::types::tendermint::{
    TendermintClientState, TendermintConsensusState,
};
use ibc_relayer::chain::client::ClientSettings;
use ibc_relayer_components::chain::traits::types::chain_id::HasChainIdType;
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
use ibc_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_relayer_components::chain::traits::types::packet::HasIbcPacketTypes;
use ibc_relayer_components::chain::traits::types::packets::ack::HasAckPacketPayload;
use ibc_relayer_components::chain::traits::types::packets::receive::HasReceivePacketPayload;
use ibc_relayer_components::chain::traits::types::packets::timeout::HasTimeoutUnorderedPacketPayload;
use ibc_relayer_components::chain::traits::types::timestamp::HasTimestampType;
use ibc_relayer_components::chain::traits::types::update_client::HasUpdateClientPayload;
use ibc_relayer_components::logger::traits::has_logger::HasLoggerType;
use ibc_relayer_components::runtime::traits::runtime::HasRuntime;
use ibc_relayer_components_extra::telemetry::traits::telemetry::HasTelemetry;
use ibc_relayer_runtime::types::error::Error as TokioError;
use ibc_relayer_runtime::types::log::logger::TracingLogger;
use ibc_relayer_runtime::types::runtime::TokioRuntimeContext;
use ibc_relayer_types::core::ics04_channel::packet::{Packet, Sequence};
use ibc_relayer_types::core::ics24_host::identifier::{
    ChainId, ChannelId, ClientId, ConnectionId, PortId,
};
use ibc_relayer_types::timestamp::Timestamp;

use crate::contexts::chain::CosmosChain;
use crate::types::error::{BaseError, Error};
use crate::types::telemetry::CosmosTelemetry;

impl<Chain> HasErrorType for CosmosChain<Chain>
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

    fn runtime_error(e: TokioError) -> Error {
        BaseError::tokio(e).into()
    }
}

impl<Chain> HasLoggerType for CosmosChain<Chain>
where
    Chain: Async,
{
    type Logger = TracingLogger;
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

impl<Chain> HasTimestampType for CosmosChain<Chain>
where
    Chain: Async,
{
    type Timestamp = Timestamp;
}

impl<Chain> HasChainIdType for CosmosChain<Chain>
where
    Chain: Async,
{
    type ChainId = ChainId;
}

impl<Chain, Counterparty> HasIbcChainTypes<Counterparty> for CosmosChain<Chain>
where
    Chain: Async,
{
    type ClientId = ClientId;

    type ConnectionId = ConnectionId;

    type ChannelId = ChannelId;

    type PortId = PortId;

    type Sequence = Sequence;
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

impl<Chain, Counterparty> HasIbcPacketTypes<Counterparty> for CosmosChain<Chain>
where
    Chain: Async,
{
    type IncomingPacket = Packet;

    type OutgoingPacket = Packet;
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
