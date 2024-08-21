use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;

use crate::chain::traits::proposal::types::proposal_id::HasProposalIdType;
use crate::chain::traits::types::amount::HasAmountType;

#[derive_component(DepositProposalMessageBuilderComponent, DepositProposalMessageBuilder<Chain>)]
pub trait CanBuildDepositProposalMessage:
    HasProposalIdType + HasAmountType + HasMessageType
{
    fn build_deposit_proposal_message(
        &self,
        proposal_id: &Self::ProposalId,
        amount: &Self::Amount,
    ) -> Self::Message;
}
