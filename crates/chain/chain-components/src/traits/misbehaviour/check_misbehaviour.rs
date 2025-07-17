use cgp::core::component::UseDelegate;
use hermes_chain_type_components::traits::HasEvidenceType;
use hermes_prelude::*;

use crate::traits::HasUpdateClientEvent;

#[cgp_component {
  provider: MisbehaviourChecker,
  context: Chain,
}]
#[async_trait]
pub trait CanCheckMisbehaviour<Counterparty>:
    HasUpdateClientEvent + HasEvidenceType + HasAsyncErrorType
{
    async fn check_misbehaviour(
        &self,
        update_event: &Self::UpdateClientEvent,
    ) -> Result<Option<Self::Evidence>, Self::Error>;
}

#[cgp_provider(MisbehaviourCheckerComponent)]
impl<Chain, Counterparty, Components> MisbehaviourChecker<Chain, Counterparty>
    for UseDelegate<Components>
where
    Chain: HasUpdateClientEvent + HasEvidenceType + HasAsyncErrorType,
    Components: DelegateComponent<Counterparty>,
    Components::Delegate: MisbehaviourChecker<Chain, Counterparty>,
{
    async fn check_misbehaviour(
        chain: &Chain,
        update_event: &Chain::UpdateClientEvent,
    ) -> Result<Option<Chain::Evidence>, Chain::Error> {
        Components::Delegate::check_misbehaviour(chain, update_event).await
    }
}
