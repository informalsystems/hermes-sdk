use alloc::vec::Vec;
use core::fmt::Debug;
use core::marker::PhantomData;

use cgp::prelude::CanRaiseError;
use hermes_chain_type_components::traits::types::time::HasTimeType;

use crate::traits::fields::packet::header::timeout::HasPacketTimeout;
use crate::traits::fields::packet::packet::header::HasPacketHeader;
use crate::traits::fields::timeout::CanCompareTimeoutTime;
use crate::traits::handlers::incoming::packet::IncomingPacketHandler;
use crate::traits::queries::time::CanQueryCurrentTime;
use crate::traits::types::commitment::proof::HasCommitmentProofType;
use crate::traits::types::packet::ack::HasPacketAckType;
use crate::traits::types::packet::packet::HasPacketType;
use crate::traits::types::packet::timeout::HasPacketTimeoutType;
use crate::types::any_app::AnyApp;

pub struct DisallowTimedOutIncomingPacket<InHandler>(pub PhantomData<InHandler>);

pub struct PacketTimedOut<'a, Chain, Counterparty>
where
    Chain: HasTimeType + HasPacketTimeoutType<Counterparty>,
    Counterparty: HasPacketType<Chain>,
{
    pub current_time: &'a Chain::Time,
    pub packet_timeout: &'a Chain::PacketTimeout,
    pub packet: &'a Counterparty::Packet,
}

impl<Chain, Counterparty, InHandler> IncomingPacketHandler<Chain, Counterparty>
    for DisallowTimedOutIncomingPacket<InHandler>
where
    Chain: CanQueryCurrentTime
        + HasPacketAckType<AnyApp, Counterparty>
        + CanCompareTimeoutTime<Counterparty>
        + for<'a> CanRaiseError<PacketTimedOut<'a, Chain, Counterparty>>,
    Counterparty: HasCommitmentProofType + HasPacketHeader<Chain> + HasPacketTimeout<Chain>,
    InHandler: IncomingPacketHandler<Chain, Counterparty>,
{
    async fn handle_incoming_packet(
        chain: &Chain,
        packet: &Counterparty::Packet,
        send_proof: &Counterparty::CommitmentProof,
    ) -> Result<Vec<Chain::PacketAck>, Chain::Error> {
        let current_time = &chain.get_current_time();

        let packet_header = Counterparty::packet_header(packet);
        let packet_timeout = Counterparty::packet_timeout(packet_header);

        if Chain::is_packet_timed_out(current_time, packet_timeout) {
            Err(Chain::raise_error(PacketTimedOut {
                current_time,
                packet_timeout,
                packet,
            }))
        } else {
            InHandler::handle_incoming_packet(chain, packet, send_proof).await
        }
    }
}

impl<'a, Chain, Counterparty> Debug for PacketTimedOut<'a, Chain, Counterparty>
where
    Chain: HasTimeType + HasPacketTimeoutType<Counterparty>,
    Counterparty: HasPacketType<Chain>,
    Chain::Time: Debug,
    Chain::PacketTimeout: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "packet timeout {:?} has exceeded the current host time {:?}",
            self.packet_timeout, self.current_time
        )?;

        Ok(())
    }
}
