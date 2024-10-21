use cgp::prelude::*;
use hermes_chain_type_components::traits::types::ibc::channel_id::HasChannelIdType;
use hermes_chain_type_components::traits::types::ibc::client_id::HasClientIdType;

#[derive_component(ClientIdFromChannelIdQuerierComponent, ClientIdFromChannelIdQuerier<Chain>)]
#[async_trait]
pub trait CanQueryClientIdFromChannelId<Counterparty>:
    HasChannelIdType<Counterparty> + HasClientIdType<Counterparty> + HasErrorType
{
    async fn query_client_id_from_channel_id(
        &self,
        channel_id: &Self::ChannelId,
    ) -> Result<Self::ClientId, Self::Error>;
}
