use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::types::aliases::{ChannelIdOf, PortIdOf};

use crate::chain::traits::types::amount::HasAmountType;
use crate::chain::traits::types::denom::HasDenomType;
use crate::chain_driver::traits::types::chain::HasChainType;

#[derive_component(RandomAmountGeneratorComponent, RandomAmountGenerator<Chain>)]
#[async_trait]
pub trait CanGenerateRandomAmount: HasChainType
where
    Self::Chain: HasDenomType + HasAmountType,
{
    async fn random_amount(&self, min: usize, max: &Self::Amount) -> Self::Amount;
}

#[derive_component(IbcTransferredAmountConverterComponent, IbcTransferredAmountConverter<Chain>)]
pub trait CanConvertIbcTransferredAmount<CounterpartyDriver>:
    HasAmountType + HasChainType + HasErrorType
where
    Self::Chain: HasIbcChainTypes<CounterpartyDriver::Chain>,
    CounterpartyDriver: HasChainType + HasAmountType,
{
    fn ibc_transfer_amount_from(
        counterparty_amount: &CounterpartyDriver::Amount,
        channel_id: &ChannelIdOf<Self::Chain, CounterpartyDriver::Chain>,
        port_id: &PortIdOf<Self::Chain, CounterpartyDriver::Chain>,
    ) -> Result<Self::Amount, Self::Error>;

    fn transmute_counterparty_amount(
        counterparty_amount: &CounterpartyDriver::Amount,
        denom: &Self::Denom,
    ) -> Self::Amount;
}
