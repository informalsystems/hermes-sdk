use cgp::prelude::*;
use hermes_chain_type_components::traits::types::amount::HasAmountType;
use hermes_chain_type_components::traits::types::ibc::channel_id::HasChannelIdType;
use hermes_chain_type_components::traits::types::quantity::HasQuantityType;
use hermes_ibc_components::traits::types::app_id::HasAppIdType;

pub struct Increase;

pub struct Decrease;

#[derive_component(EscrowedTokenUpdateComponent, EscrowedTokenLookuper<Chain>)]
#[async_trait]
pub trait CanUpdateEscrowedToken<Counterparty, Mode: Async>:
    HasAmountType
    + HasQuantityType
    + HasChannelIdType<Counterparty>
    + HasAppIdType<Counterparty>
    + HasErrorType
where
    Counterparty: HasChannelIdType<Self> + HasAppIdType<Self>,
{
    async fn update_escrowed_token(
        &self,
        mode: Mode,
        local_channel_id: &Self::ChannelId,
        counterparty_channel_id: &Counterparty::ChannelId,
        local_app_id: &Self::AppId,
        counterparty_app_id: &Counterparty::AppId,
        amount: &Self::Amount,
    ) -> Result<(), Self::Error>;
}
