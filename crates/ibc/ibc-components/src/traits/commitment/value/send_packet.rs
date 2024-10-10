use cgp::prelude::*;

use crate::traits::types::commitment::value::HasCommitmentValueType;
use crate::traits::types::packet::packet::HasPacketType;

#[derive_component(SendPacketCommitmentValueBuilderComponent, SendPacketCommitmentValueBuilder<Chain>)]
pub trait CanBuildSendPacketCommitmentValue<Counterparty>:
    HasPacketType<Counterparty> + HasCommitmentValueType + HasErrorType
{
    // Note: this may be called by the counterparty chain, thus the lack of access to &self.
    fn build_send_packet_commitment_value(
        packet: &Self::Packet,
    ) -> Result<Self::CommitmentValue, Self::Error>;
}

#[derive_component(RecvPacketCommitmentValueBuilderComponent, RecvPacketCommitmentValueBuilder<Chain>)]
pub trait CanBuildRecvPacketCommitmentValue<Counterparty>:
    HasCommitmentValueType + HasErrorType
where
    Counterparty: HasPacketType<Self>,
{
    // Note: this may be called by the counterparty chain, thus the lack of access to &self.
    fn build_recv_packet_commitment_value(
        packet: &Counterparty::Packet,
    ) -> Result<Self::CommitmentValue, Self::Error>;
}
