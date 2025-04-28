use cgp::prelude::*;
use hermes_chain_type_components::traits::{HasAmountType, HasChannelIdType, HasQuantityType};
use hermes_ibc_components::traits::types::app_id::HasAppIdType;

/**
   A token transfer module may escrow tokens on behalf of multiple
   channel/app pairs. The escrow registry is used to track the
   amount escrowed for each channel/app pair.

   When `register_escrowed_token` is called, the registry should
   increase the escrowed amount registered with the given channel/app
   pair.
*/
#[cgp_component {
  provider: EscrowTokenRegistrar,
  context: Chain,
}]
#[async_trait]
pub trait CanRegisterEscrowToken<Counterparty>:
    HasAmountType
    + HasQuantityType
    + HasChannelIdType<Counterparty>
    + HasAppIdType<Counterparty>
    + HasAsyncErrorType
where
    Counterparty: HasChannelIdType<Self> + HasAppIdType<Self>,
{
    async fn register_escrowed_token(
        &mut self,
        src_channel_id: &Self::ChannelId,
        dst_channel_id: &Counterparty::ChannelId,
        src_app_id: &Self::AppId,
        dst_app_id: &Counterparty::AppId,
        amount: &Self::Amount,
    ) -> Result<(), Self::Error>;
}
