use core::cmp::Ordering;
use core::fmt::{Debug, Display};

use cgp::prelude::*;
use hermes_chain_type_components::traits::{
    AmountBuilder, AmountBuilderComponent, AmountDenomGetter, AmountDenomGetterComponent,
    AmountQuantityGetter, AmountQuantityGetterComponent, AmountTypeProvider,
    AmountTypeProviderComponent,
};

use crate::components::chain::MockChainComponents;
use crate::contexts::chain::MockChain;
use crate::types::denom::MockDenom;
use crate::types::quantity::MockQuantity;

pub struct MockAmount<Chain, Counterparty> {
    pub quantity: MockQuantity,
    pub denom: MockDenom<Chain, Counterparty>,
}

#[cgp_provider(AmountTypeProviderComponent)]
impl<Chain: Async, Counterparty: Async> AmountTypeProvider<MockChain<Chain, Counterparty>>
    for MockChainComponents
{
    type Amount = MockAmount<Chain, Counterparty>;
}

#[cgp_provider(AmountDenomGetterComponent)]
impl<Chain: Async, Counterparty: Async> AmountDenomGetter<MockChain<Chain, Counterparty>>
    for MockChainComponents
{
    fn amount_denom(amount: &MockAmount<Chain, Counterparty>) -> &MockDenom<Chain, Counterparty> {
        &amount.denom
    }
}

#[cgp_provider(AmountQuantityGetterComponent)]
impl<Chain: Async, Counterparty: Async> AmountQuantityGetter<MockChain<Chain, Counterparty>>
    for MockChainComponents
{
    fn amount_quantity(amount: &MockAmount<Chain, Counterparty>) -> &MockQuantity {
        &amount.quantity
    }
}

#[cgp_provider(AmountBuilderComponent)]
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

impl<Chain, Counterparty> PartialOrd for MockAmount<Chain, Counterparty> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<Chain, Counterparty> Ord for MockAmount<Chain, Counterparty> {
    fn cmp(&self, other: &Self) -> Ordering {
        let ordering_a = self.quantity.cmp(&other.quantity);
        let ordering_b = self.denom.cmp(&other.denom);

        ordering_a.then(ordering_b)
    }
}
