use cgp::prelude::*;
use hermes_chain_type_components::traits::types::amount::HasAmountType;
use hermes_ibc_components::traits::types::message::HasIbcMessageType;
use hermes_ibc_components::traits::types::message_header::HasIbcMessageHeaderType;
use hermes_ibc_components::traits::types::payload::data::HasPayloadDataType;
use hermes_ibc_components::traits::types::payload::header::HasPayloadHeaderType;

#[cgp_component {
  name: UnescrowPayloadBuilderComponent,
  provider: UnescrowPayloadBuilder,
  context: Chain,
}]
pub trait CanBuildUnescrowPayload<Counterparty, App>:
    HasPayloadHeaderType<Counterparty>
    + HasPayloadDataType<Counterparty, App>
    + HasIbcMessageHeaderType<Counterparty>
    + HasIbcMessageType<Counterparty, App>
    + HasErrorType
where
    Counterparty: HasAmountType,
{
    fn build_outgoing_unescrow_payload(
        &self,
        message_header: &Self::IbcMessageHeader,
        message: &Self::IbcMessage,
        amount: &Counterparty::Amount,
    ) -> Result<(Self::PayloadHeader, Self::PayloadData), Self::Error>;
}
