use borsh::BorshSerialize;
use hermes_cosmos_chain_components::traits::message::CosmosMessage;
use ibc_relayer_types::signer::Signer;

use crate::types::messages::bank::BankMessage;
use crate::types::messages::ibc::{IbcMessage, IbcMessageWithHeight};

#[derive(Debug, BorshSerialize)]
pub enum SovereignMessage {
    Accounts,
    Bank(BankMessage),
    Ibc(IbcMessage),
}

impl From<CosmosMessage> for SovereignMessage {
    fn from(cosmos_message: CosmosMessage) -> Self {
        let cosmos_message_any = cosmos_message.message.encode_protobuf(&Signer::dummy());

        IbcMessageWithHeight::new(
            cosmos_message_any,
            cosmos_message
                .message
                .counterparty_message_height_for_update_client(),
        )
        .into()
    }
}
