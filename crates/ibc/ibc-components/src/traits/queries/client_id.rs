use cgp::prelude::*;
use hermes_chain_type_components::traits::{HasChannelIdType, HasClientIdType};

#[cgp_component {
  provider: ClientIdFromChannelIdQuerier,
  context: Chain,
}]
#[async_trait]
pub trait CanQueryClientIdFromChannelId<Counterparty>:
    HasChannelIdType<Counterparty> + HasClientIdType<Counterparty> + HasAsyncErrorType
{
    async fn query_client_id_from_channel_id(
        &self,
        channel_id: &Self::ChannelId,
    ) -> Result<Self::ClientId, Self::Error>;
}
