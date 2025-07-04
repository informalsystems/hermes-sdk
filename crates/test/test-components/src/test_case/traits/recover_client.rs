use hermes_chain_type_components::traits::HasClientIdType;
use hermes_prelude::*;

#[cgp_component {
  provider: RecoverClientHandler,
  context: Driver,
}]
#[async_trait]
pub trait CanHandleRecoverClient<ChainDriverA, ChainA, ChainB>: HasAsyncErrorType
where
    ChainA: HasClientIdType<ChainB>,
{
    async fn handle_recover_client(
        &self,
        subject_client_id: &ChainA::ClientId,
        substitute_client_id: &ChainA::ClientId,
    ) -> Result<(), Self::Error>;
}
