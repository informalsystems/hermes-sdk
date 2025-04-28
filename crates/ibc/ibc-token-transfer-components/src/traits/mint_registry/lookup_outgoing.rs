use hermes_chain_type_components::traits::{HasChannelIdType, HasDenomType};
use hermes_ibc_components::traits::types::app_id::HasAppIdType;
use hermes_prelude::*;

#[cgp_component {
  provider: OutgoingBurnTokenQuerier,
  context: Chain,
}]
#[async_trait]
pub trait CanLookupOutgoingBurnToken<Counterparty>:
    HasDenomType + HasChannelIdType<Counterparty> + HasAppIdType<Counterparty> + HasAsyncErrorType
where
    Counterparty: HasDenomType + HasChannelIdType<Self> + HasAppIdType<Self>,
{
    async fn lookup_outgoing_burn_token(
        &self,
        src_channel_id: &Self::ChannelId,
        dst_channel_id: &Counterparty::ChannelId,
        src_app_id: &Self::AppId,
        dst_app_id: &Counterparty::AppId,
        src_denom: &Self::Denom,
    ) -> Result<Option<Counterparty::Denom>, Self::Error>;
}
