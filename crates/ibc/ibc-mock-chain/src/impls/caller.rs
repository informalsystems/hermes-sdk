use hermes_ibc_components::traits::fields::caller::{CallerGetter, CallerGetterComponent};
use hermes_prelude::*;

use crate::components::chain::MockChainComponents;
use crate::contexts::chain::MockChain;
use crate::types::address::MockAddress;
use crate::types::tagged::Tagged;

#[cgp_provider(CallerGetterComponent)]
impl<Chain: Async, Counterparty: Async> CallerGetter<MockChain<Chain, Counterparty>>
    for MockChainComponents
{
    fn caller(chain: &MockChain<Chain, Counterparty>) -> Tagged<Chain, Counterparty, MockAddress> {
        chain.current_caller.clone()
    }
}
