use cgp::prelude::*;
use hermes_chain_type_components::traits::types::amount::HasAmountType;
use hermes_chain_type_components::traits::types::ibc::channel_id::HasChannelIdType;
use hermes_chain_type_components::traits::types::quantity::HasQuantityType;
use hermes_ibc_components::traits::types::app_id::HasAppIdType;

#[derive_component(EscrowTokenRegistrarComponent, EscrowTokenRegistrar<Chain>)]
#[async_trait]
pub trait CanRegisterEscrowToken<Counterparty>:
    HasAmountType
    + HasQuantityType
    + HasChannelIdType<Counterparty>
    + HasAppIdType<Counterparty>
    + HasErrorType
where
    Counterparty: HasChannelIdType<Self> + HasAppIdType<Self>,
{
    async fn register_escrowed_token(
        &self,
        src_channel_id: &Self::ChannelId,
        dst_channel_id: &Counterparty::ChannelId,
        src_app_id: &Self::AppId,
        dst_app_id: &Counterparty::AppId,
        amount: &Self::Amount,
    ) -> Result<(), Self::Error>;
}
