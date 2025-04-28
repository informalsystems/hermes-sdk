use cgp::core::component::UseDelegate;
use hermes_prelude::*;
use hermes_relayer_components::chain::traits::{HasHeightType, HasTimeoutType};

#[cgp_component {
    provider: IbcTransferTimeoutCalculator,
    context: Chain,
}]
pub trait CanCalculateIbcTransferTimeout<Counterparty>
where
    Counterparty: HasTimeoutType + HasHeightType,
{
    fn ibc_transfer_timeout_time(
        &self,
        current_time: &Counterparty::Time,
    ) -> Option<Counterparty::Timeout>;

    fn ibc_transfer_timeout_height(
        &self,
        current_height: &Counterparty::Height,
    ) -> Option<Counterparty::Height>;
}

#[cgp_provider(IbcTransferTimeoutCalculatorComponent)]
impl<Chain, Counterparty, Components> IbcTransferTimeoutCalculator<Chain, Counterparty>
    for UseDelegate<Components>
where
    Counterparty: HasTimeoutType + HasHeightType,
    Components: DelegateComponent<Counterparty>,
    Components::Delegate: IbcTransferTimeoutCalculator<Chain, Counterparty>,
{
    fn ibc_transfer_timeout_time(
        chain: &Chain,
        current_time: &Counterparty::Time,
    ) -> Option<Counterparty::Timeout> {
        Components::Delegate::ibc_transfer_timeout_time(chain, current_time)
    }

    fn ibc_transfer_timeout_height(
        chain: &Chain,
        current_height: &Counterparty::Height,
    ) -> Option<Counterparty::Height> {
        Components::Delegate::ibc_transfer_timeout_height(chain, current_height)
    }
}
