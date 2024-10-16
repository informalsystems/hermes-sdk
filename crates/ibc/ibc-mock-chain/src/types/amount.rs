use core::fmt::{Debug, Display};

use cgp::core::Async;
use hermes_chain_type_components::traits::types::amount::ProvideAmountType;

use crate::contexts::chain::MockChain;
use crate::types::denom::MockDenom;
use crate::types::quantity::MockQuantity;
use crate::types::tagged::Tagged;

pub struct MockAmount<Chain, Counterparty> {
    pub quantity: Tagged<Chain, Counterparty, MockQuantity>,
    pub denom: MockDenom<Chain, Counterparty>,
}

pub struct UseMockAmountType;

impl<Chain: Async, Counterparty: Async> ProvideAmountType<MockChain<Chain, Counterparty>>
    for UseMockAmountType
{
    type Amount = MockAmount<Chain, Counterparty>;
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
            quantity: self.quantity.clone(),
            denom: self.denom.clone(),
        }
    }
}
