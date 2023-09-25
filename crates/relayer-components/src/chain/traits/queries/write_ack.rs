use async_trait::async_trait;

use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::chain::traits::types::ibc_events::write_ack::HasWriteAcknowledgementEvent;
use crate::chain::traits::types::packet::HasIbcPacketTypes;
use crate::std_prelude::*;
use cgp_core::traits::error::HasErrorType;

#[async_trait]
pub trait CanQueryWriteAcknowledgement<Counterparty>:
    HasWriteAcknowledgementEvent<Counterparty> + HasIbcPacketTypes<Counterparty> + HasErrorType
where
    Counterparty: HasIbcChainTypes<Self>,
{
    async fn query_write_acknowledgement_event(
        &self,
        packet: &Self::IncomingPacket,
    ) -> Result<Option<Self::WriteAcknowledgementEvent>, Self::Error>;
}
