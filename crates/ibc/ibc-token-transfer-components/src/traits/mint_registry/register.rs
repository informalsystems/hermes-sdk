use cgp::prelude::*;
use hermes_chain_type_components::traits::types::denom::HasDenomType;
use hermes_chain_type_components::traits::types::ibc::channel_id::HasChannelIdType;
use hermes_ibc_components::traits::types::app_id::HasAppIdType;

#[cgp_component {
  provider: MintedTokenRegistrar,
  context: Chain,
}]
#[async_trait]
pub trait CanRegisterMintedToken<Counterparty>:
    HasDenomType + HasChannelIdType<Counterparty> + HasAppIdType<Counterparty> + HasAsyncErrorType
where
    Counterparty: HasDenomType + HasChannelIdType<Self> + HasAppIdType<Self>,
{
    async fn register_minted_token(
        &self,
        src_channel_id: &Counterparty::ChannelId,
        dst_channel_id: &Self::ChannelId,
        src_app_id: &Counterparty::AppId,
        dst_app_id: &Self::AppId,
        src_denom: &Counterparty::Denom,
        dst_denom: &Self::Denom,
    ) -> Result<(), Self::Error>;
}
