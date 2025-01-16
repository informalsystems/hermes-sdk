use cgp::prelude::*;

use crate::traits::types::commitment::value::HasCommitmentValueType;
use crate::traits::types::packet::packet::HasPacketType;
use crate::types::tags::commitment::send::SendPacket;

#[cgp_component {
  provider: SendPacketCommitmentValueBuilder,
  context: Chain,
}]
pub trait CanBuildSendPacketCommitmentValue<Counterparty>:
    HasPacketType<Counterparty> + HasCommitmentValueType<SendPacket> + HasAsyncErrorType
{
    // Note: this may be called by the counterparty chain, thus the lack of access to &self.
    fn build_send_packet_commitment_value(
        packet: &Self::Packet,
    ) -> Result<Self::CommitmentValue, Self::Error>;
}
