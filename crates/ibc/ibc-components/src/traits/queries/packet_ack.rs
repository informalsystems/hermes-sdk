use alloc::vec::Vec;
use cgp::prelude::*;
use hermes_chain_type_components::traits::types::ibc::client_id::HasClientIdType;

use crate::traits::types::packet::ack::HasPacketAckType;
use crate::traits::types::packet::nonce::HasPacketNonceType;

#[async_trait]
pub trait CanQueryPacketAck<Counterparty, App>:
    HasPacketAckType<Counterparty, App> + HasClientIdType<Counterparty> + HasErrorType
where
    Counterparty: HasClientIdType<Self> + HasPacketNonceType<Self>,
{
    async fn query_packet_ack(
        &self,
        src_client_id: &Counterparty::ClientId,
        dst_client_id: &Self::ClientId,
        nonce: &Counterparty::PacketNonce,
    ) -> Result<Option<Vec<Self::PacketAck>>, Self::Error>;
}
