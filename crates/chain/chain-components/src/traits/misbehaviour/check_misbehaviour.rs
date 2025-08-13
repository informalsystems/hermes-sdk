use cgp::core::component::UseDelegate;
use hermes_chain_type_components::traits::{HasClientStateType, HasEvidenceType};
use hermes_prelude::*;

use crate::traits::HasUpdateClientEvent;

#[cgp_component {
  provider: MisbehaviourChecker,
  context: Chain,
}]
#[async_trait]
pub trait CanCheckMisbehaviour<Counterparty>:
    HasClientStateType<Counterparty> + HasAsyncErrorType
where
    Counterparty: HasEvidenceType + HasUpdateClientEvent,
{
    async fn check_misbehaviour(
        &self,
        update_event: &Counterparty::UpdateClientEvent,
        client_state: &Self::ClientState,
    ) -> Result<Option<Counterparty::Evidence>, Self::Error>;
}

#[cgp_provider(MisbehaviourCheckerComponent)]
impl<Chain, Counterparty, Components> MisbehaviourChecker<Chain, Counterparty>
    for UseDelegate<Components>
where
    Chain: HasClientStateType<Counterparty> + HasAsyncErrorType,
    Counterparty: HasEvidenceType + HasUpdateClientEvent,
    Components: DelegateComponent<Counterparty>,
    Components::Delegate: MisbehaviourChecker<Chain, Counterparty>,
{
    async fn check_misbehaviour(
        chain: &Chain,
        update_event: &Counterparty::UpdateClientEvent,
        client_state: &Chain::ClientState,
    ) -> Result<Option<Counterparty::Evidence>, Chain::Error> {
        Components::Delegate::check_misbehaviour(chain, update_event, client_state).await
    }
}
