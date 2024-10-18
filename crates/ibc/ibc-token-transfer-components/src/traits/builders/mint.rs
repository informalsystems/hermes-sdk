use cgp::prelude::*;
use hermes_chain_type_components::traits::types::amount::HasAmountType;
use hermes_ibc_components::traits::types::message::HasIbcMessageType;
use hermes_ibc_components::traits::types::message_header::HasIbcMessageHeaderType;
use hermes_ibc_components::traits::types::payload::data::HasPayloadDataType;
use hermes_ibc_components::traits::types::payload::header::HasPayloadHeaderType;

#[derive_component(OutgoingMintPayloadBuilderComponent, OutgoingMintPayloadBuilder<Chain>)]
pub trait CanBuildOutgoingMintPayload<Counterparty, App>:
    HasPayloadHeaderType<Counterparty>
    + HasPayloadDataType<Counterparty, App>
    + HasIbcMessageHeaderType<Counterparty>
    + HasIbcMessageType<Counterparty, App>
    + HasAmountType
    + HasErrorType
{
    fn build_outgoing_mint_payload(
        &self,
        message_header: &Self::IbcMessageHeader,
        message: &Self::IbcMessage,
        amount: &Self::Amount,
    ) -> Result<(Self::PayloadHeader, Self::PayloadData), Self::Error>;
}
