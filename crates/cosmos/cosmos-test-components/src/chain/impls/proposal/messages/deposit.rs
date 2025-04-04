use cgp::prelude::*;
use hermes_chain_type_components::traits::types::amount::HasAmountType;
use hermes_cosmos_chain_components::traits::message::{
    CosmosMessage, DynCosmosMessage, ToCosmosMessage,
};
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use hermes_test_components::chain::traits::proposal::messages::deposit::{
    DepositProposalMessageBuilder, DepositProposalMessageBuilderComponent,
};
use hermes_test_components::chain::traits::proposal::types::proposal_id::HasProposalIdType;
use ibc::primitives::Signer;
use ibc_proto::cosmos::base::v1beta1::Coin;
use ibc_proto::cosmos::gov::v1::MsgDeposit;
use ibc_proto::google::protobuf::Any;
use prost::{Message, Name};

use crate::chain::types::amount::Amount;

pub struct BuildDepositProposalMessage;

#[derive(Debug)]
pub struct DepositMessage {
    pub proposal_id: u64,
    pub amount: Coin,
}

#[cgp_provider(DepositProposalMessageBuilderComponent)]
impl<Chain> DepositProposalMessageBuilder<Chain> for BuildDepositProposalMessage
where
    Chain: HasProposalIdType<ProposalId = u64>
        + HasAmountType<Amount = Amount>
        + HasMessageType<Message = CosmosMessage>,
{
    fn build_deposit_proposal_message(
        _chain: &Chain,
        proposal_id: &u64,
        amount: &Amount,
    ) -> CosmosMessage {
        let message = DepositMessage {
            proposal_id: *proposal_id,
            amount: Coin {
                denom: amount.denom.to_string(),
                amount: amount.quantity.to_string(),
            },
        };

        message.to_cosmos_message()
    }
}

impl DynCosmosMessage for DepositMessage {
    fn encode_protobuf(&self, signer: &Signer) -> Any {
        let message = MsgDeposit {
            proposal_id: self.proposal_id,
            amount: vec![self.amount.clone()],
            depositor: signer.to_string(),
        };

        Any {
            type_url: MsgDeposit::type_url(),
            value: message.encode_to_vec(),
        }
    }
}
