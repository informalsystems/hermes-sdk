use hermes_core::chain_type_components::traits::{HasAddressType, HasAmountType};
use hermes_core::relayer_components::chain::traits::HasMessageType;
use hermes_cosmos_chain_components::traits::{CosmosMessage, DynCosmosMessage, ToCosmosMessage};
use hermes_cosmos_test_components::chain::types::Amount;
use hermes_prelude::*;
use ibc::primitives::Signer;
use ibc_proto::cosmos::base::v1beta1::Coin;
use ibc_proto::cosmos::gov::v1::MsgSubmitProposal;
use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::lightclients::wasm::v1::MsgStoreCode;
use prost::Message;

use crate::traits::chain::{StoreCodeMessageBuilder, StoreCodeMessageBuilderComponent};

pub struct BuildStoreCodeMessage;

#[derive(Debug)]
pub struct StoreCodeProposalMessage {
    pub wasm_byte_code: Vec<u8>,
    pub title: String,
    pub summary: String,
    pub authority: String,
    pub deposit_amount: Amount,
}

#[cgp_provider(StoreCodeMessageBuilderComponent)]
impl<Chain> StoreCodeMessageBuilder<Chain> for BuildStoreCodeMessage
where
    Chain:
        HasAmountType<Amount = Amount> + HasAddressType + HasMessageType<Message = CosmosMessage>,
{
    fn build_store_code_message(
        _chain: &Chain,
        wasm_byte_code: &Vec<u8>,
        title: &str,
        summary: &str,
        authority: &Chain::Address,
        deposit_amount: &Amount,
    ) -> CosmosMessage {
        let message = StoreCodeProposalMessage {
            wasm_byte_code: wasm_byte_code.clone(),
            title: title.into(),
            summary: summary.into(),
            authority: authority.to_string(),
            deposit_amount: deposit_amount.clone(),
        };

        message.to_cosmos_message()
    }
}

impl DynCosmosMessage for StoreCodeProposalMessage {
    fn encode_protobuf(&self, signer: &Signer) -> Any {
        let store_code_message = MsgStoreCode {
            signer: self.authority.clone(),
            wasm_byte_code: self.wasm_byte_code.clone(),
        };

        let store_code_message_any = Any {
            type_url: "/ibc.lightclients.wasm.v1.MsgStoreCode".into(),
            value: store_code_message.encode_to_vec(),
        };

        let proposal_message = MsgSubmitProposal {
            messages: vec![store_code_message_any],
            initial_deposit: vec![Coin {
                denom: self.deposit_amount.denom.to_string(),
                amount: self.deposit_amount.quantity.to_string(),
            }],
            proposer: signer.to_string(),
            metadata: "".into(),
            title: self.title.clone(),
            summary: self.summary.clone(),
            expedited: false,
        };

        Any {
            type_url: "/cosmos.gov.v1.MsgSubmitProposal".into(),
            value: proposal_message.encode_to_vec(),
        }
    }
}
