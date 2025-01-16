use cgp::prelude::*;
use hermes_chain_type_components::traits::types::denom::HasDenomType;
use hermes_chain_type_components::traits::types::ibc::channel_id::HasChannelIdType;
use hermes_ibc_components::traits::types::app_id::HasAppIdType;

/**
   Create a new token address used for minting an IBC denom with the given channels.

   The given parameters may be ignored, in the case when the token creation is
   implemented by deploying a new token contract. Otherwise, the parameters
   may be optionally used to deterministically derive a local denom.
*/
#[cgp_component {
  provider: TokenCreator,
  context: Chain,
}]
#[async_trait]
pub trait CanCreateToken<Counterparty>:
    HasDenomType + HasChannelIdType<Counterparty> + HasAppIdType<Counterparty> + HasAsyncErrorType
where
    Counterparty: HasDenomType + HasChannelIdType<Self> + HasAppIdType<Self>,
{
    async fn create_token(
        &self,
        src_channel_id: &Counterparty::ChannelId,
        dst_channel_id: &Self::ChannelId,
        src_app_id: &Counterparty::AppId,
        dst_app_id: &Self::AppId,
        src_denom: &Counterparty::Denom,
    ) -> Result<Self::Denom, Self::Error>;
}
