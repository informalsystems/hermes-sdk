use cgp_core::prelude::*;
use cgp_core::HasErrorType;

use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::chain::traits::types::ibc_events::write_ack::HasWriteAckEvent;
use crate::chain::traits::types::packet::HasIbcPacketTypes;
use crate::std_prelude::*;

#[derive_component(WriteAckQuerierComponent, WriteAckQuerier<Chain>)]
#[async_trait]
pub trait CanQueryWriteAck<Counterparty>:
    HasWriteAckEvent<Counterparty> + HasIbcPacketTypes<Counterparty> + HasErrorType
where
    Counterparty: HasIbcChainTypes<Self>,
{
    async fn query_write_ack_event(
        &self,
        packet: &Self::IncomingPacket,
    ) -> Result<Option<Self::WriteAckEvent>, Self::Error>;
}
