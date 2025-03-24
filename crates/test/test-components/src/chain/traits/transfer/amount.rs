use core::marker::PhantomData;

use cgp::core::component::UseDelegate;
use cgp::prelude::*;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;

use crate::chain::traits::types::amount::HasAmountType;

#[cgp_component {
    provider: IbcTransferredAmountConverter,
    context: Chain,
}]
#[async_trait]
pub trait CanConvertIbcTransferredAmount<Counterparty>:
    HasAmountType + HasIbcChainTypes<Counterparty> + HasAsyncErrorType
where
    Counterparty: HasAmountType,
{
    async fn ibc_transfer_amount_from(
        &self,
        _counterparty: PhantomData<Counterparty>,
        counterparty_amount: &Counterparty::Amount,
        channel_id: &Self::ChannelId,
        port_id: &Self::PortId,
    ) -> Result<Self::Amount, Self::Error>;

    async fn transmute_counterparty_amount(
        &self,
        _counterparty: PhantomData<Counterparty>,
        counterparty_amount: &Counterparty::Amount,
        denom: &Self::Denom,
    ) -> Result<Self::Amount, Self::Error>;
}

#[cgp_provider(IbcTransferredAmountConverterComponent)]
impl<Chain, Counterparty, Components> IbcTransferredAmountConverter<Chain, Counterparty>
    for UseDelegate<Components>
where
    Chain: HasAmountType + HasIbcChainTypes<Counterparty> + HasAsyncErrorType,
    Counterparty: HasAmountType,
    Components: DelegateComponent<Counterparty>,
    Components::Delegate: IbcTransferredAmountConverter<Chain, Counterparty>,
{
    async fn ibc_transfer_amount_from(
        chain: &Chain,
        counterparty: PhantomData<Counterparty>,
        counterparty_amount: &Counterparty::Amount,
        channel_id: &Chain::ChannelId,
        port_id: &Chain::PortId,
    ) -> Result<Chain::Amount, Chain::Error> {
        Components::Delegate::ibc_transfer_amount_from(
            chain,
            counterparty,
            counterparty_amount,
            channel_id,
            port_id,
        )
        .await
    }

    async fn transmute_counterparty_amount(
        chain: &Chain,
        counterparty: PhantomData<Counterparty>,
        counterparty_amount: &Counterparty::Amount,
        denom: &Chain::Denom,
    ) -> Result<Chain::Amount, Chain::Error> {
        Components::Delegate::transmute_counterparty_amount(
            chain,
            counterparty,
            counterparty_amount,
            denom,
        )
        .await
    }
}
