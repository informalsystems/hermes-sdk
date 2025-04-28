use core::fmt::Debug;
use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_chain_type_components::traits::HasTimeType;

use crate::traits::fields::packet::header::timeout::HasPacketTimeout;
use crate::traits::fields::packet::packet::header::HasPacketHeader;
use crate::traits::fields::timeout::CanCompareTimeoutTime;
use crate::traits::handlers::incoming::packet::{
    IncomingPacketHandler, IncomingPacketHandlerComponent,
};
use crate::traits::queries::time::CanQueryCurrentTime;
use crate::traits::types::commitment::proof::HasCommitmentProofType;
use crate::traits::types::packet::packet::HasPacketType;
use crate::traits::types::packet::timeout::HasPacketTimeoutType;
use crate::types::tags::commitment::send::SendPacket;

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

#[cgp_provider(IncomingPacketHandlerComponent)]
#[async_trait]
impl<Chain, Counterparty, InHandler> IncomingPacketHandler<Chain, Counterparty>
    for DisallowTimedOutIncomingPacket<InHandler>
where
    Chain: CanQueryCurrentTime
        + CanCompareTimeoutTime<Counterparty>
        + for<'a> CanRaiseAsyncError<PacketTimedOut<'a, Chain, Counterparty>>,
    Counterparty:
        HasCommitmentProofType<SendPacket> + HasPacketHeader<Chain> + HasPacketTimeout<Chain>,
    InHandler: IncomingPacketHandler<Chain, Counterparty>,
{
    async fn handle_incoming_packet(
        chain: &mut Chain,
        packet: &Counterparty::Packet,
        send_proof: &Counterparty::CommitmentProof,
    ) -> Result<(), Chain::Error> {
        let current_time = &chain.get_current_time().await;

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

impl<Chain, Counterparty> Debug for PacketTimedOut<'_, Chain, Counterparty>
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
