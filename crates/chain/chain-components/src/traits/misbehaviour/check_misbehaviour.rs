use hermes_chain_type_components::traits::{HasClientStateType, HasEventType};
use hermes_prelude::*;

use crate::traits::HasEvidenceType;

#[cgp_component {
  provider: MisbehaviourChecker,
  context: Chain,
}]
#[async_trait]
pub trait CanCheckMisbehaviour<Counterparty>:
    HasEventType + HasClientStateType<Counterparty> + HasEvidenceType + HasAsyncErrorType
{
    async fn check_misbehaviour(
        &self,
        update_event: &Self::Event,
        client_state: &Self::ClientState,
    ) -> Result<Option<Self::Evidence>, Self::Error>;
}
