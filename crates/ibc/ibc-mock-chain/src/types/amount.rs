use core::fmt::{Debug, Display};

use cgp::core::Async;
use hermes_chain_type_components::traits::builders::amount::AmountBuilder;
use hermes_chain_type_components::traits::fields::amount::denom::AmountDenomGetter;
use hermes_chain_type_components::traits::fields::amount::quantity::AmountQuantityGetter;
use hermes_chain_type_components::traits::types::amount::ProvideAmountType;

use crate::components::chain::MockChainComponents;
use crate::contexts::chain::MockChain;
use crate::types::denom::MockDenom;
use crate::types::quantity::MockQuantity;

pub struct MockAmount<Chain, Counterparty> {
    pub quantity: MockQuantity,
    pub denom: MockDenom<Chain, Counterparty>,
}

impl<Chain: Async, Counterparty: Async> ProvideAmountType<MockChain<Chain, Counterparty>>
    for MockChainComponents
{
    type Amount = MockAmount<Chain, Counterparty>;
}

impl<Chain: Async, Counterparty: Async> AmountDenomGetter<MockChain<Chain, Counterparty>>
    for MockChainComponents
{
    fn amount_denom(amount: &MockAmount<Chain, Counterparty>) -> &MockDenom<Chain, Counterparty> {
        &amount.denom
    }
}

impl<Chain: Async, Counterparty: Async> AmountQuantityGetter<MockChain<Chain, Counterparty>>
    for MockChainComponents
{
    fn amount_quantity(amount: &MockAmount<Chain, Counterparty>) -> &MockQuantity {
        &amount.quantity
    }
}

impl<Chain: Async, Counterparty: Async> AmountBuilder<MockChain<Chain, Counterparty>>
    for MockChainComponents
{
    fn build_amount(
        denom: &MockDenom<Chain, Counterparty>,
        quantity: &MockQuantity,
    ) -> MockAmount<Chain, Counterparty> {
        MockAmount {
            denom: denom.clone(),
            quantity: *quantity,
        }
    }
}

impl<Chain, Counterparty> Debug for MockAmount<Chain, Counterparty> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("MockAmount")
            .field("quantity", &self.quantity)
            .field("denom", &self.denom)
            .finish()
    }
}

impl<Chain, Counterparty> Display for MockAmount<Chain, Counterparty> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl<Chain, Counterparty> Clone for MockAmount<Chain, Counterparty> {
    fn clone(&self) -> Self {
        Self {
            quantity: self.quantity,
            denom: self.denom.clone(),
        }
    }
}

impl<Chain, Counterparty> PartialEq for MockAmount<Chain, Counterparty> {
    fn eq(&self, other: &Self) -> bool {
        self.quantity == other.quantity && self.denom == other.denom
    }
}

impl<Chain, Counterparty> Eq for MockAmount<Chain, Counterparty> {}
