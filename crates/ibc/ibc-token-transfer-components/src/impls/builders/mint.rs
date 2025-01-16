use cgp::prelude::HasAsyncErrorType;
use hermes_chain_type_components::traits::types::address::HasAddressType;
use hermes_chain_type_components::traits::types::amount::HasAmountType;
use hermes_ibc_components::traits::types::message::HasIbcMessageType;
use hermes_ibc_components::traits::types::message_header::HasIbcMessageHeaderType;
use hermes_ibc_components::traits::types::payload::data::HasPayloadDataType;
use hermes_ibc_components::traits::types::payload::header::HasPayloadHeaderType;

use crate::traits::builders::mint::MintPayloadBuilder;
use crate::traits::fields::message::receiver::HasMessageTransferReceiver;
use crate::types::packet_data::mint::IbcTransferMintPayloadData;

pub struct BuildMintPayload;

impl<Chain, Counterparty, App> MintPayloadBuilder<Chain, Counterparty, App> for BuildMintPayload
where
    Chain: HasPayloadHeaderType<Counterparty>
        + HasPayloadDataType<Counterparty, App>
        + HasIbcMessageHeaderType<Counterparty, IbcMessageHeader: Clone>
        + HasIbcMessageType<Counterparty, App>
        + HasMessageTransferReceiver<Counterparty, App>
        + HasAmountType<Amount: Clone>
        + HasAsyncErrorType,
    Counterparty: HasAmountType<Amount: Clone> + HasAddressType<Address: Clone>,
    Chain::PayloadHeader: From<Chain::IbcMessageHeader>,
    Chain::PayloadData: From<IbcTransferMintPayloadData<Chain, Counterparty>>,
{
    fn build_outgoing_mint_payload(
        _chain: &Chain,
        message_header: &Chain::IbcMessageHeader,
        message: &Chain::IbcMessage,
        amount: &Chain::Amount,
    ) -> Result<(Chain::PayloadHeader, Chain::PayloadData), Chain::Error> {
        let receiver = Chain::message_transfer_receiver(message);

        let payload_data = IbcTransferMintPayloadData {
            receiver: receiver.clone(),
            amount: amount.clone(),
        };

        Ok((message_header.clone().into(), payload_data.into()))
    }
}
