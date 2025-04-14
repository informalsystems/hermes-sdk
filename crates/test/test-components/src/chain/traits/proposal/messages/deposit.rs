use cgp::prelude::*;
use hermes_chain_type_components::traits::HasAmountType;
use hermes_relayer_components::chain::traits::HasMessageType;

use crate::chain::traits::HasProposalIdType;

#[cgp_component {
  provider: DepositProposalMessageBuilder,
  context: Chain,
}]
pub trait CanBuildDepositProposalMessage:
    HasProposalIdType + HasAmountType + HasMessageType
{
    fn build_deposit_proposal_message(
        &self,
        proposal_id: &Self::ProposalId,
        amount: &Self::Amount,
    ) -> Self::Message;
}
