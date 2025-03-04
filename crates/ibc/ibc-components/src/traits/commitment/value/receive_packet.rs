use cgp::prelude::*;

use crate::traits::types::commitment::value::HasCommitmentValueType;
use crate::traits::types::packet::packet::HasPacketType;
use crate::types::tags::commitment::receive::ReceivePacket;

#[cgp_component {
  provider: ReceivePacketCommitmentValueBuilder,
  context: Chain,
}]
pub trait CanBuildReceivePacketCommitmentValue<Counterparty>:
    HasCommitmentValueType<ReceivePacket> + HasAsyncErrorType
where
    Counterparty: HasPacketType<Self>,
{
    // Note: this may be called by the counterparty chain, thus the lack of access to &self.
    fn build_receive_packet_commitment_value(
        packet: &Counterparty::Packet,
    ) -> Result<Self::CommitmentValue, Self::Error>;
}
