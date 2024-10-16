use alloc::borrow::ToOwned;
use alloc::string::String;
use cgp::core::Async;
use hermes_ibc_token_transfer_components::traits::token::transfer::{Mint, TokenTransferer};

use crate::components::chain::MockChainComponents;
use crate::contexts::chain::MockChain;
use crate::types::address::MockAddress;
use crate::types::amount::MockAmount;
use crate::types::tagged::Tagged;

impl<Chain: Async, Counterparty: Async> TokenTransferer<MockChain<Chain, Counterparty>, Mint>
    for MockChainComponents
{
    async fn transfer_token(
        chain: &MockChain<Chain, Counterparty>,
        _mode: Mint,
        target: &Tagged<Chain, Counterparty, MockAddress>,
        amount: &MockAmount<Chain, Counterparty>,
    ) -> Result<(), String> {
        let mut lock = chain.state.lock().await;
        let state = lock.mock_chain_state_mut();

        let denom_balance = state
            .balances
            .entry(amount.denom.clone())
            .or_insert_with(Default::default);

        let target_balance = denom_balance
            .entry(target.clone())
            .or_insert_with(Default::default);

        target_balance.0 = target_balance
            .0
            .checked_add(amount.quantity.0)
            .ok_or_else(|| "add quantity overflow".to_owned())?;

        Ok(())
    }
}
