use cgp::prelude::*;
use hermes_chain_type_components::traits::types::denom::HasDenomType;
use hermes_chain_type_components::traits::types::ibc::channel_id::HasChannelIdType;
use hermes_ibc_components::traits::types::app_id::HasAppIdType;

#[cgp_component {
  provider: IncomingMintedTokenQuerier,
  context: Chain,
}]
#[async_trait]
pub trait CanLookupIncomingMintedToken<Counterparty>:
    HasDenomType + HasChannelIdType<Counterparty> + HasAppIdType<Counterparty> + HasErrorType
where
    Counterparty: HasDenomType + HasChannelIdType<Self> + HasAppIdType<Self>,
{
    async fn lookup_incoming_minted_token(
        &self,
        src_channel_id: &Counterparty::ChannelId,
        dst_channel_id: &Self::ChannelId,
        src_app_id: &Counterparty::AppId,
        dst_app_id: &Self::AppId,
        src_denom: &Counterparty::Denom,
    ) -> Result<Option<Self::Denom>, Self::Error>;
}
