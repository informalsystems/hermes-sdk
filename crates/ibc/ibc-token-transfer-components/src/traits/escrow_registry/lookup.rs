use cgp::prelude::*;
use hermes_chain_type_components::traits::types::denom::HasDenomType;
use hermes_chain_type_components::traits::types::ibc::channel_id::HasChannelIdType;
use hermes_chain_type_components::traits::types::quantity::HasQuantityType;
use hermes_ibc_components::traits::types::app_id::HasAppIdType;

#[derive_component(EscrowedTokenLookuperComponent, EscrowedTokenLookuper<Chain>)]
#[async_trait]
pub trait CanLookupEscrowedToken<Counterparty>:
    HasDenomType
    + HasQuantityType
    + HasChannelIdType<Counterparty>
    + HasAppIdType<Counterparty>
    + HasErrorType
where
    Counterparty: HasChannelIdType<Self> + HasAppIdType<Self>,
{
    async fn lookup_escrowed_token(
        &self,
        src_channel_id: &Counterparty::ChannelId,
        dst_channel_id: &Self::ChannelId,
        src_app_id: &Counterparty::AppId,
        dst_app_id: &Self::AppId,
        denom: &Self::Denom,
    ) -> Result<Option<Self::Quantity>, Self::Error>;
}
