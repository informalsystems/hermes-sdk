use cgp::core::component::UseDelegate;
use hermes_chain_type_components::traits::{HasEvidenceType, HasMessageType};
use hermes_prelude::*;

#[cgp_component {
  provider: MisbehaviourMessageBuilder,
  context: Chain,
}]
#[async_trait]
pub trait CanBuildMisbehaviourMessage<Counterparty>:
    HasEvidenceType + HasMessageType + HasAsyncErrorType
{
    async fn build_misbehaviour_message(
        &self,
        evidence: &Self::Evidence,
    ) -> Result<Self::Message, Self::Error>;
}

#[cgp_provider(MisbehaviourMessageBuilderComponent)]
impl<Chain, Counterparty, Components> MisbehaviourMessageBuilder<Chain, Counterparty>
    for UseDelegate<Components>
where
    Chain: HasMessageType + HasEvidenceType + HasAsyncErrorType,
    Components: DelegateComponent<Counterparty>,
    Components::Delegate: MisbehaviourMessageBuilder<Chain, Counterparty>,
{
    async fn build_misbehaviour_message(
        chain: &Chain,
        evidence: &Chain::Evidence,
    ) -> Result<Chain::Message, Chain::Error> {
        Components::Delegate::build_misbehaviour_message(chain, evidence).await
    }
}
