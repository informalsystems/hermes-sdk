use cgp::prelude::HasErrorType;
use hermes_chain_type_components::traits::fields::amount::denom::HasAmountDenom;
use hermes_chain_type_components::traits::fields::amount::quantity::HasAmountQuantity;
use hermes_chain_type_components::traits::types::denom::HasDenomType;
use hermes_chain_type_components::traits::types::ibc::channel_id::HasChannelIdType;
use hermes_ibc_components::traits::fields::message::app_id::HasIbcMessageAppIds;
use hermes_ibc_components::traits::fields::transaction::caller::HasIbcTransactionCaller;
use hermes_ibc_components::traits::fields::transaction::channel_id::HasIbcTransactionChannelIds;
use hermes_ibc_components::traits::handlers::outgoing::message::IbcMessageHandler;
use hermes_ibc_components::traits::types::app_id::HasAppIdType;
use hermes_ibc_components::traits::types::message::HasIbcMessageType;
use hermes_ibc_components::traits::types::message_header::HasIbcMessageHeaderType;
use hermes_ibc_components::traits::types::payload::data::HasPayloadDataType;
use hermes_ibc_components::traits::types::payload::header::HasPayloadHeaderType;
use hermes_ibc_components::traits::types::transaction_header::HasIbcTransactionHeaderType;

use crate::traits::fields::message::amount::HasMessageSendTransferAmount;
use crate::traits::mint_registry::lookup_outgoing::CanLookupOutgoingBurnToken;
use crate::traits::token::transfer::{Burn, CanTransferToken, Escrow};

pub struct SendIbcTransfer;

impl<Chain, Counterparty, App> IbcMessageHandler<Chain, Counterparty, App> for SendIbcTransfer
where
    Chain: HasErrorType
        + HasIbcTransactionHeaderType<Counterparty>
        + HasIbcMessageHeaderType<Counterparty>
        + HasIbcMessageType<Counterparty, App>
        + HasPayloadDataType<Counterparty, App>
        + HasPayloadHeaderType<Counterparty>
        + HasIbcTransactionChannelIds<Counterparty>
        + HasIbcMessageAppIds<Counterparty>
        + HasMessageSendTransferAmount<Counterparty, App>
        + HasAmountDenom
        + HasAmountQuantity
        + HasIbcTransactionCaller<Counterparty>
        + CanLookupOutgoingBurnToken<Counterparty>
        + CanTransferToken<Burn>
        + CanTransferToken<Escrow>,
    Counterparty: HasDenomType + HasChannelIdType<Chain> + HasAppIdType<Chain>,
{
    async fn handle_ibc_message(
        chain: &Chain,
        transaction_header: &Chain::IbcTransactionHeader,
        message_header: &Chain::IbcMessageHeader,
        message: &Chain::IbcMessage,
    ) -> Result<(Chain::PayloadHeader, Chain::PayloadData), Chain::Error> {
        let src_channel_id = Chain::transaction_src_channel_id(transaction_header);
        let dst_channel_id = Chain::transaction_dst_channel_id(transaction_header);

        let src_app_id = Chain::ibc_message_src_app_id(message_header);
        let dst_app_id = Chain::ibc_message_dst_app_id(message_header);

        let src_amount = Chain::message_send_transfer_amount(message);
        let src_denom = Chain::amount_denom(src_amount);

        let sender = Chain::ibc_transaction_caller(transaction_header);

        let m_dst_denom = chain
            .lookup_outgoing_burn_token(
                src_channel_id,
                dst_channel_id,
                src_app_id,
                dst_app_id,
                src_denom,
            )
            .await?;

        if let Some(dst_denom) = m_dst_denom {
            chain.transfer_token(Burn, sender, src_amount).await?;

            todo!()
        } else {
            chain.transfer_token(Escrow, sender, src_amount).await?;

            todo!()
        }
    }
}
