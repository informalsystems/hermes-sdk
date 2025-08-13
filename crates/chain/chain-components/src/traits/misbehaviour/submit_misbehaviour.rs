use cgp::core::component::UseDelegate;
use hermes_chain_type_components::traits::{HasClientIdType, HasEvidenceType, HasMessageType};
use hermes_prelude::*;

#[cgp_component {
  provider: MisbehaviourMessageBuilder,
  context: Chain,
}]
#[async_trait]
pub trait CanBuildMisbehaviourMessage<Counterparty>:
    HasEvidenceType + HasClientIdType<Counterparty> + HasMessageType + HasAsyncErrorType
{
    async fn build_misbehaviour_message(
        &self,
        client_id: &Self::ClientId,
        evidence: &Self::Evidence,
    ) -> Result<Self::Message, Self::Error>;
}

#[cgp_provider(MisbehaviourMessageBuilderComponent)]
impl<Chain, Counterparty, Components, Delegate> MisbehaviourMessageBuilder<Chain, Counterparty>
    for UseDelegate<Components>
where
    Chain: HasMessageType + HasClientIdType<Counterparty> + HasEvidenceType + HasAsyncErrorType,
    Delegate: MisbehaviourMessageBuilder<Chain, Counterparty>,
    Components: DelegateComponent<Counterparty, Delegate = Delegate>,
{
    async fn build_misbehaviour_message(
        chain: &Chain,
        client_id: &Chain::ClientId,
        evidence: &Chain::Evidence,
    ) -> Result<Chain::Message, Chain::Error> {
        Components::Delegate::build_misbehaviour_message(chain, client_id, evidence).await
    }
}
