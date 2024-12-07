use cgp::prelude::*;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;

use crate::chain::traits::proposal::types::proposal_id::HasProposalIdType;
use crate::chain::traits::types::amount::HasAmountType;

#[cgp_component {
  name: DepositProposalMessageBuilderComponent,
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
