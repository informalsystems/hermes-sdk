use hermes_chain_components::traits::HasUpdateClientEvent;
use hermes_chain_components::types::aliases::UpdateClientEventOf;
use hermes_prelude::*;

use crate::relay::traits::{HasTargetChainTypes, RelayTarget};

#[cgp_component {
  provider: MisbehaviourCheckerAndSubmitter,
  context: Relay,
}]
#[async_trait]
pub trait CanCheckAndSubmitMisbehaviour<Target: RelayTarget>:
    HasTargetChainTypes<Target, TargetChain: HasUpdateClientEvent + Async, CounterpartyChain: Async>
    + HasAsyncErrorType
{
    /**
       Check misbehaviour using update client events and submit evidence if found.
    */
    async fn check_and_submit_misbehaviour(
        &self,
        target: Target,
        target_chain: &Self::TargetChain,
        counterparty_chain: &Self::CounterpartyChain,
        update_client_event: &UpdateClientEventOf<Self::TargetChain>,
    ) -> Result<(), Self::Error>;
}
