use cgp::prelude::*;
use hermes_chain_type_components::traits::types::amount::HasAmountType;
use hermes_chain_type_components::traits::types::ibc::channel_id::HasChannelIdType;
use hermes_ibc_components::traits::types::app_id::HasAppIdType;

/**
   A token transfer module may escrow tokens on behalf of multiple
   channel/app pairs. The escrow registry is used to track the
   amount escrowed for each channel/app pair.

   When `register_unescrow_token` is called, the registry should
   first check whether the given channel/app pair has sufficient
   amount escrowed. If so, it would decrement the amount escrowed.
*/
#[derive_component(UnescrowTokenRegistrarComponent, UnescrowTokenRegistrar<Chain>)]
#[async_trait]
pub trait CanRegisterUnescrowToken<Counterparty>:
    HasAmountType + HasChannelIdType<Counterparty> + HasAppIdType<Counterparty> + HasErrorType
where
    Counterparty: HasChannelIdType<Self> + HasAppIdType<Self>,
{
    async fn register_unescrow_token(
        &self,
        src_channel_id: &Counterparty::ChannelId,
        dst_channel_id: &Self::ChannelId,
        src_app_id: &Counterparty::AppId,
        dst_app_id: &Self::AppId,
        amount: &Self::Amount,
    ) -> Result<(), Self::Error>;
}
