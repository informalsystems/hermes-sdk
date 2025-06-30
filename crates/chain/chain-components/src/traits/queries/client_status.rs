use core::marker::PhantomData;

use cgp::core::component::UseDelegate;
use hermes_chain_type_components::traits::{HasClientIdType, HasClientStatusType, HasHeightType};
use hermes_prelude::*;

#[cgp_component {
  provider: ClientStatusQuerier,
  context: Chain,
}]
#[async_trait]
pub trait CanQueryClientStatus<Counterparty>:
    HasClientIdType<Counterparty> + HasHeightType + HasAsyncErrorType
where
    Counterparty: HasClientStatusType<Self>,
{
    async fn query_client_status(
        &self,
        tag: PhantomData<Counterparty>,
        client_id: &Self::ClientId,
    ) -> Result<Counterparty::ClientStatus, Self::Error>;
}

#[cgp_provider(ClientStatusQuerierComponent)]
impl<Chain, Counterparty, Components, Delegate> ClientStatusQuerier<Chain, Counterparty>
    for UseDelegate<Components>
where
    Chain: HasHeightType + HasClientIdType<Counterparty> + HasAsyncErrorType,
    Counterparty: HasClientStatusType<Chain>,
    Components: DelegateComponent<Counterparty, Delegate = Delegate>,
    Delegate: ClientStatusQuerier<Chain, Counterparty>,
{
    async fn query_client_status(
        chain: &Chain,
        tag: PhantomData<Counterparty>,
        client_id: &Chain::ClientId,
    ) -> Result<Counterparty::ClientStatus, Chain::Error> {
        Delegate::query_client_status(chain, tag, client_id).await
    }
}
