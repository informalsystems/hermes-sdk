use cgp::prelude::*;
use hermes_chain_type_components::traits::types::ibc::client_id::HasClientIdType;

use crate::traits::types::app_id::HasAppIdType;
use crate::traits::types::message::HasIbcMessageType;
use crate::traits::types::packet::data::HasPacketDataType;

#[derive_component(OutgoingPacketDataBuilderComponent, OutgoingPacketDataBuilder<Chain>)]
pub trait CanBuildOutgoingPacketData<App, Counterparty>:
    HasErrorType
    + HasIbcMessageType<App, Counterparty>
    + HasPacketDataType<App, Counterparty>
    + HasClientIdType<Counterparty>
    + HasAppIdType<Counterparty>
where
    Counterparty: HasClientIdType<Self>,
{
    fn build_outgoing_packet_data(
        &self,
        source_client_id: &Self::ClientId,
        destination_client_id: &Counterparty::ClientId,
        source_app_id: &Self::AppId,
        message: &Self::IbcMessage,
    ) -> Result<Self::PacketData, Self::Error>;
}
