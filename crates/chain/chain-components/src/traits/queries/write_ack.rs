use cgp::prelude::*;
use hermes_chain_type_components::traits::types::ibc::packet::HasOutgoingPacketType;

use crate::traits::types::ibc_events::write_ack::HasWriteAckEvent;

#[derive_component(WriteAckQuerierComponent, WriteAckQuerier<Chain>)]
#[async_trait]
pub trait CanQueryWriteAck<Counterparty>: HasWriteAckEvent<Counterparty> + HasErrorType
where
    Counterparty: HasOutgoingPacketType<Self>,
{
    async fn query_write_ack_event(
        &self,
        packet: &Counterparty::OutgoingPacket,
    ) -> Result<Option<Self::WriteAckEvent>, Self::Error>;
}
