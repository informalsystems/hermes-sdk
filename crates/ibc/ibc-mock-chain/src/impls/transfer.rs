use alloc::borrow::ToOwned;
use alloc::string::String;

use cgp::prelude::*;
use hermes_ibc_token_transfer_components::traits::token::transfer::{
    Burn, Escrow, Mint, TokenTransferer, TokenTransfererComponent, Unescrow,
};

use crate::components::chain::MockChainComponents;
use crate::contexts::chain::MockChain;
use crate::types::address::MockAddress;
use crate::types::amount::MockAmount;
use crate::types::tagged::Tagged;

#[cgp_provider(TokenTransfererComponent)]
impl<Chain: Async, Counterparty: Async> TokenTransferer<MockChain<Chain, Counterparty>, Mint>
    for MockChainComponents
{
    async fn transfer_token(
        chain: &mut MockChain<Chain, Counterparty>,
        _mode: Mint,
        target: &Tagged<Chain, Counterparty, MockAddress>,
        amount: &MockAmount<Chain, Counterparty>,
    ) -> Result<(), String> {
        let state = chain.pending_state.mock_chain_state_mut();

        let denom_balance = state.balances.entry(amount.denom.clone()).or_default();

        let target_balance = denom_balance.entry(target.clone()).or_default();

        target_balance.value = target_balance
            .value
            .checked_add(amount.quantity.value)
            .ok_or_else(|| "add quantity overflow".to_owned())?;

        Ok(())
    }
}

#[cgp_provider(TokenTransfererComponent)]
impl<Chain: Async, Counterparty: Async> TokenTransferer<MockChain<Chain, Counterparty>, Unescrow>
    for MockChainComponents
{
    async fn transfer_token(
        chain: &mut MockChain<Chain, Counterparty>,
        _mode: Unescrow,
        target: &Tagged<Chain, Counterparty, MockAddress>,
        amount: &MockAmount<Chain, Counterparty>,
    ) -> Result<(), String> {
        let state = chain.pending_state.mock_chain_state_mut();

        let denom_balance = state.balances.entry(amount.denom.clone()).or_default();

        let source_balance = denom_balance
            .entry(MockAddress::TransferApp.into())
            .or_default();

        source_balance.value = source_balance
            .value
            .checked_sub(amount.quantity.value)
            .ok_or_else(|| "transfer app has insufficient fund to unescrow".to_owned())?;

        let target_balance = denom_balance.entry(target.clone()).or_default();

        target_balance.value = target_balance
            .value
            .checked_add(amount.quantity.value)
            .ok_or_else(|| "add quantity overflow".to_owned())?;

        Ok(())
    }
}

#[cgp_provider(TokenTransfererComponent)]
impl<Chain: Async, Counterparty: Async> TokenTransferer<MockChain<Chain, Counterparty>, Escrow>
    for MockChainComponents
{
    async fn transfer_token(
        chain: &mut MockChain<Chain, Counterparty>,
        _mode: Escrow,
        target: &Tagged<Chain, Counterparty, MockAddress>,
        amount: &MockAmount<Chain, Counterparty>,
    ) -> Result<(), String> {
        let state = chain.pending_state.mock_chain_state_mut();

        let denom_balance = state.balances.entry(amount.denom.clone()).or_default();

        let source_balance = denom_balance.entry(target.clone()).or_default();

        source_balance.value = source_balance
            .value
            .checked_sub(amount.quantity.value)
            .ok_or_else(|| "user has insufficient fund to unescrow".to_owned())?;

        let target_balance = denom_balance
            .entry(MockAddress::TransferApp.into())
            .or_default();

        target_balance.value = target_balance
            .value
            .checked_add(amount.quantity.value)
            .ok_or_else(|| "add quantity overflow".to_owned())?;

        Ok(())
    }
}

#[cgp_provider(TokenTransfererComponent)]
impl<Chain: Async, Counterparty: Async> TokenTransferer<MockChain<Chain, Counterparty>, Burn>
    for MockChainComponents
{
    async fn transfer_token(
        chain: &mut MockChain<Chain, Counterparty>,
        _mode: Burn,
        target: &Tagged<Chain, Counterparty, MockAddress>,
        amount: &MockAmount<Chain, Counterparty>,
    ) -> Result<(), String> {
        let state = chain.pending_state.mock_chain_state_mut();

        let denom_balance = state.balances.entry(amount.denom.clone()).or_default();

        let target_balance = denom_balance.entry(target.clone()).or_default();

        target_balance.value = target_balance
            .value
            .checked_sub(amount.quantity.value)
            .ok_or_else(|| "user has insufficient balance".to_owned())?;

        Ok(())
    }
}
