use cgp::prelude::*;
use hermes_ibc_components::traits::fields::commitment::proof_height::{
    CommitmentProofHeightGetter, CommitmentProofHeightGetterComponent,
};
use hermes_ibc_components::traits::types::commitment::proof::{
    CommitmentProofTypeComponent, ProvideCommitmentProofType,
};

use crate::components::chain::MockChainComponents;
use crate::contexts::chain::MockChain;
use crate::types::height::MockHeight;
use crate::types::tagged::Tagged;

pub struct MockCommitmentProof<Chain, Counterparty> {
    pub height: Tagged<Chain, Counterparty, MockHeight>,
}

#[cgp_provider(CommitmentProofTypeComponent)]
impl<Chain: Async, Counterparty: Async, Tag>
    ProvideCommitmentProofType<MockChain<Chain, Counterparty>, Tag> for MockChainComponents
{
    type CommitmentProof = MockCommitmentProof<Chain, Counterparty>;
}

#[cgp_provider(CommitmentProofHeightGetterComponent)]
impl<Chain: Async, Counterparty: Async, Tag>
    CommitmentProofHeightGetter<MockChain<Chain, Counterparty>, Tag> for MockChainComponents
{
    fn commitment_proof_height(
        proof: &MockCommitmentProof<Chain, Counterparty>,
    ) -> &Tagged<Chain, Counterparty, MockHeight> {
        &proof.height
    }
}
