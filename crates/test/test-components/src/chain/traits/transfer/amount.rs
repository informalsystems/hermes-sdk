use core::marker::PhantomData;

use cgp::core::component::UseDelegate;
use cgp::prelude::*;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;

use crate::chain::traits::types::amount::HasAmountType;

#[cgp_component {
    provider: IbcTransferredAmountConverter,
    context: Chain,
}]
pub trait CanConvertIbcTransferredAmount<Counterparty>:
    HasAmountType + HasIbcChainTypes<Counterparty> + HasAsyncErrorType
where
    Counterparty: HasAmountType,
{
    fn ibc_transfer_amount_from(
        _counterparty: PhantomData<Counterparty>,
        counterparty_amount: &Counterparty::Amount,
        channel_id: &Self::ChannelId,
        port_id: &Self::PortId,
    ) -> Result<Self::Amount, Self::Error>;

    fn transmute_counterparty_amount(
        _counterparty: PhantomData<Counterparty>,
        counterparty_amount: &Counterparty::Amount,
        denom: &Self::Denom,
    ) -> Self::Amount;
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
    fn ibc_transfer_amount_from(
        counterparty: PhantomData<Counterparty>,
        counterparty_amount: &Counterparty::Amount,
        channel_id: &Chain::ChannelId,
        port_id: &Chain::PortId,
    ) -> Result<Chain::Amount, Chain::Error> {
        Components::Delegate::ibc_transfer_amount_from(
            counterparty,
            counterparty_amount,
            channel_id,
            port_id,
        )
    }

    fn transmute_counterparty_amount(
        counterparty: PhantomData<Counterparty>,
        counterparty_amount: &Counterparty::Amount,
        denom: &Chain::Denom,
    ) -> Chain::Amount {
        Components::Delegate::transmute_counterparty_amount(
            counterparty,
            counterparty_amount,
            denom,
        )
    }
}
