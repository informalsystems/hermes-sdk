use alloc::vec::Vec;

use cgp_core::prelude::*;

use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::chain::traits::types::ibc_events::write_ack::HasWriteAckEvent;
use crate::chain::traits::types::packet::HasIbcPacketTypes;
use crate::chain::types::aliases::WriteAckEventOf;

#[derive_component(AckPacketsQuerierComponent, AckPacketsQuerier<Chain>)]
#[async_trait]
pub trait CanQueryAckPackets<Counterparty>:
    HasIbcChainTypes<Counterparty>
    + HasIbcPacketTypes<Counterparty>
    + HasWriteAckEvent<Counterparty>
    + HasErrorType
where
    Counterparty: HasIbcChainTypes<Self>,
{
    /// Given a list of sequences, a channel and port will query a list of outgoing
    /// ack packets which have not been relayed.
    async fn query_ack_packets_from_sequences(
        &self,
        channel_id: &Counterparty::ChannelId,
        port_id: &Counterparty::PortId,
        counterparty_channel_id: &Self::ChannelId,
        counterparty_port_id: &Self::PortId,
        sequences: &[Self::Sequence],
        // The height is given to query the packets from a specific height.
        // This height should be the same as the query height from the
        // `CanQueryPacketAcknowledgements` made on the same chain.
        height: &Self::Height,
    ) -> Result<Vec<(Self::OutgoingPacket, WriteAckEventOf<Self, Counterparty>)>, Self::Error>;
}

#[derive_component(AckPacketQuerierComponent, AckPacketQuerier<Chain>)]
#[async_trait]
pub trait CanQueryAckPacket<Counterparty>:
    HasIbcChainTypes<Counterparty>
    + HasIbcPacketTypes<Counterparty>
    + HasWriteAckEvent<Counterparty>
    + HasErrorType
where
    Counterparty: HasIbcChainTypes<Self>,
{
    /// Given a list of sequences, a channel and port will query a list of outgoing
    /// ack packets which have not been relayed.
    async fn query_ack_packet_from_sequence(
        &self,
        channel_id: &Counterparty::ChannelId,
        port_id: &Counterparty::PortId,
        counterparty_channel_id: &Self::ChannelId,
        counterparty_port_id: &Self::PortId,
        sequence: &Self::Sequence,
        height: &Self::Height,
    ) -> Result<(Self::OutgoingPacket, WriteAckEventOf<Self, Counterparty>), Self::Error>;
}
