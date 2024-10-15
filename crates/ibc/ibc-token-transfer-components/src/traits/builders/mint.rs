use cgp::prelude::*;
use hermes_ibc_components::traits::types::message::HasIbcMessageType;
use hermes_ibc_components::traits::types::message_header::HasIbcMessageHeaderType;
use hermes_ibc_components::traits::types::payload::payload::HasPayloadType;

#[derive_component(OutgoingMintPayloadBuilderComponent, OutgoingMintPayloadBuilder<Chain>)]
pub trait CanBuildOutgoingMintPayload<Counterparty, App>:
    HasPayloadType<Counterparty>
    + HasIbcMessageHeaderType<Counterparty>
    + HasIbcMessageType<Counterparty, App>
    + HasErrorType
{
    fn build_outgoing_mint_payload(
        &self,
        message_header: &Self::IbcMessageHeader,
        message: &Self::IbcMessage,
    ) -> Result<Self::Payload, Self::Error>;
}
