use alloc::vec::Vec;
use core::marker::PhantomData;

use cgp::prelude::{DelegateComponent, HasErrorType};

use crate::traits::queries::client_state::{
    AllClientStatesQuerier, ClientStateQuerier, ClientStateWithProofsQuerier,
};
use crate::traits::types::client_state::HasClientStateType;
use crate::traits::types::ibc::HasIbcChainTypes;
use crate::traits::types::proof::HasCommitmentProofType;

pub struct DelegateQueryClientState<Components>(pub PhantomData<Components>);

impl<Chain, Counterparty, Components, Delegate> ClientStateQuerier<Chain, Counterparty>
    for DelegateQueryClientState<Components>
where
    Chain: HasIbcChainTypes<Counterparty> + HasErrorType,
    Counterparty: HasClientStateType<Chain>,
    Components: DelegateComponent<Counterparty, Delegate = Delegate>,
    Delegate: ClientStateQuerier<Chain, Counterparty>,
{
    async fn query_client_state(
        chain: &Chain,
        client_id: &Chain::ClientId,
        height: &Chain::Height,
    ) -> Result<Counterparty::ClientState, Chain::Error> {
        Delegate::query_client_state(chain, client_id, height).await
    }
}

impl<Chain, Counterparty, Components, Delegate> ClientStateWithProofsQuerier<Chain, Counterparty>
    for DelegateQueryClientState<Components>
where
    Chain: HasIbcChainTypes<Counterparty> + HasCommitmentProofType + HasErrorType,
    Counterparty: HasClientStateType<Chain>,
    Components: DelegateComponent<Counterparty, Delegate = Delegate>,
    Delegate: ClientStateWithProofsQuerier<Chain, Counterparty>,
{
    async fn query_client_state_with_proofs(
        chain: &Chain,
        client_id: &Chain::ClientId,
        height: &Chain::Height,
    ) -> Result<(Counterparty::ClientState, Chain::CommitmentProof), Chain::Error> {
        Delegate::query_client_state_with_proofs(chain, client_id, height).await
    }
}

impl<Chain, Counterparty, Components, Delegate> AllClientStatesQuerier<Chain, Counterparty>
    for DelegateQueryClientState<Components>
where
    Chain: HasIbcChainTypes<Counterparty> + HasErrorType,
    Counterparty: HasClientStateType<Chain>,
    Components: DelegateComponent<Counterparty, Delegate = Delegate>,
    Delegate: AllClientStatesQuerier<Chain, Counterparty>,
{
    async fn query_all_client_states(
        chain: &Chain,
        height: &Chain::Height,
    ) -> Result<Vec<(Chain::ClientId, Counterparty::ClientState)>, Chain::Error> {
        Delegate::query_all_client_states(chain, height).await
    }
}
