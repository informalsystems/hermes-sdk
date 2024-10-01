use cgp::prelude::*;

use crate::traits::types::packet::ack::HasPacketAckType;

#[derive_component(ErrorAsAckWrapperComponent, ErrorAsAckWrapper<Chain>)]
pub trait CanWrapErrorAsAck<Counterparty>: HasErrorType + HasPacketAckType<Counterparty> {
    fn try_wrap_error_as_ack(e: Self::Error) -> Result<Self::PacketAck, Self::Error>;
}
