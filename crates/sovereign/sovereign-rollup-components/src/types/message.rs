use borsh::BorshSerialize;

use hermes_cosmos_chain_components::traits::message::CosmosMessage;
use ibc_relayer_types::signer::Signer;

use crate::types::messages::bank::BankMessage;
use crate::types::messages::ibc::IbcMessage;

#[derive(BorshSerialize)]
pub enum SovereignMessage {
    Accounts,
    Bank(BankMessage),
    Ibc(IbcMessage),
}

impl From<CosmosMessage> for SovereignMessage {
    fn from(cosmos_message: CosmosMessage) -> Self {
        let cosmos_message_any = cosmos_message.message.encode_protobuf(&Signer::dummy());
        let ibc_message = IbcMessage::Core(cosmos_message_any);
        SovereignMessage::Ibc(ibc_message)
    }
}
