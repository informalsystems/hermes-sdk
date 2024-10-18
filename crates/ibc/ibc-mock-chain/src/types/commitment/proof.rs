use cgp::core::Async;
use hermes_ibc_components::traits::fields::commitment::proof_height::CommitmentProofHeightGetter;
use hermes_ibc_components::traits::types::commitment::proof::ProvideCommitmentProofType;

use crate::components::chain::MockChainComponents;
use crate::contexts::chain::MockChain;
use crate::types::height::MockHeight;
use crate::types::tagged::Tagged;

pub struct MockCommitmentProof<Chain, Counterparty> {
    pub height: Tagged<Chain, Counterparty, MockHeight>,
}

impl<Chain: Async, Counterparty: Async, Tag>
    ProvideCommitmentProofType<MockChain<Chain, Counterparty>, Tag> for MockChainComponents
{
    type CommitmentProof = MockCommitmentProof<Chain, Counterparty>;
}

impl<Chain: Async, Counterparty: Async, Tag>
    CommitmentProofHeightGetter<MockChain<Chain, Counterparty>, Tag> for MockChainComponents
{
    fn commitment_proof_height(
        proof: &MockCommitmentProof<Chain, Counterparty>,
    ) -> &Tagged<Chain, Counterparty, MockHeight> {
        &proof.height
    }
}
