use ibc_relayer_components::chain::traits::components::chain_status_querier::CanQueryChainStatus;
use ibc_relayer_components::chain::traits::components::consensus_state_querier::CanQueryConsensusState;
use ibc_relayer_components::chain::traits::components::packet_commitments_querier::CanQueryPacketCommitments;
use ibc_relayer_components::chain::traits::components::packet_fields_reader::CanReadPacketFields;
use ibc_relayer_components::chain::traits::components::received_packet_querier::CanQueryReceivedPacket;
use ibc_relayer_components::chain::traits::components::send_packets_querier::CanQuerySendPackets;
use ibc_relayer_components::chain::traits::components::unreceived_packet_sequences_querier::CanQueryUnreceivedPacketSequences;
use ibc_relayer_components::chain::traits::types::chain::HasChainTypes;
use ibc_relayer_components::chain::traits::types::channel::{
    HasChannelHandshakePayloads, HasInitChannelOptionsType,
};
use ibc_relayer_components::chain::traits::types::connection::{
    HasConnectionHandshakePayloads, HasInitConnectionOptionsType,
};
use ibc_relayer_components::chain::traits::types::consensus_state::HasConsensusStateType;
use ibc_relayer_components::chain::traits::types::create_client::HasCreateClientOptions;
use ibc_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_relayer_components::chain::traits::types::ibc_events::write_ack::HasWriteAckEvent;
use ibc_relayer_components::chain::traits::types::packet::HasIbcPacketTypes;
use ibc_relayer_components::logger::traits::level::HasLoggerWithBaseLevels;
use ibc_relayer_components_extra::telemetry::traits::telemetry::HasTelemetry;

use crate::all_for_one::runtime::HasAfoRuntime;

pub trait AfoChain<Counterparty>:
    Clone
    + HasAfoRuntime
    + HasLoggerWithBaseLevels
    + HasTelemetry
    + HasChainTypes
    + CanQueryChainStatus
    + HasIbcChainTypes<Counterparty>
    + CanReadPacketFields<Counterparty>
    + HasWriteAckEvent<Counterparty>
    + HasConsensusStateType<Counterparty>
    + CanQueryConsensusState<Counterparty>
    + CanQueryReceivedPacket<Counterparty>
    + CanQueryPacketCommitments<Counterparty>
    + CanQueryUnreceivedPacketSequences<Counterparty>
    + CanQuerySendPackets<Counterparty>
    + HasCreateClientOptions<Counterparty>
    + HasInitConnectionOptionsType<Counterparty>
    + HasConnectionHandshakePayloads<Counterparty>
    + HasInitChannelOptionsType<Counterparty>
    + HasChannelHandshakePayloads<Counterparty>
where
    Counterparty: AfoCounterpartyChain<Self>,
{
}

pub trait AfoCounterpartyChain<Chain>:
    HasIbcChainTypes<Chain>
    + HasConsensusStateType<Chain>
    + HasIbcPacketTypes<
        Chain,
        IncomingPacket = Chain::OutgoingPacket,
        OutgoingPacket = Chain::IncomingPacket,
    >
where
    Chain: CanReadPacketFields<Self>,
{
}

impl<Chain, Counterparty> AfoChain<Counterparty> for Chain
where
    Counterparty: AfoCounterpartyChain<Self>,
    Chain: Clone
        + HasAfoRuntime
        + HasLoggerWithBaseLevels
        + HasTelemetry
        + HasChainTypes
        + CanQueryChainStatus
        + CanReadPacketFields<Counterparty>
        + HasWriteAckEvent<Counterparty>
        + HasConsensusStateType<Counterparty>
        + CanQueryConsensusState<Counterparty>
        + CanQueryReceivedPacket<Counterparty>
        + CanQueryPacketCommitments<Counterparty>
        + CanQueryUnreceivedPacketSequences<Counterparty>
        + CanQuerySendPackets<Counterparty>
        + HasCreateClientOptions<Counterparty>
        + HasInitConnectionOptionsType<Counterparty>
        + HasConnectionHandshakePayloads<Counterparty>
        + HasInitChannelOptionsType<Counterparty>
        + HasChannelHandshakePayloads<Counterparty>,
{
}

impl<Chain, Counterparty> AfoCounterpartyChain<Chain> for Counterparty
where
    Chain: CanReadPacketFields<Counterparty>,
    Counterparty: HasConsensusStateType<Chain>
        + CanReadPacketFields<
            Chain,
            IncomingPacket = Chain::OutgoingPacket,
            OutgoingPacket = Chain::IncomingPacket,
        >,
{
}
