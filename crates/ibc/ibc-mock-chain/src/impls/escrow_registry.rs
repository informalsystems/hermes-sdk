use alloc::borrow::ToOwned;
use alloc::string::String;

use hermes_ibc_token_transfer_components::traits::escrow_registry::escrow::{
    EscrowTokenRegistrar, EscrowTokenRegistrarComponent,
};
use hermes_ibc_token_transfer_components::traits::escrow_registry::unescrow::{
    UnescrowTokenRegistrar, UnescrowTokenRegistrarComponent,
};
use hermes_prelude::*;

use crate::components::chain::MockChainComponents;
use crate::contexts::chain::MockChain;
use crate::types::amount::MockAmount;
use crate::types::app_id::MockAppId;
use crate::types::channel_id::MockChannelId;
use crate::types::tagged::Tagged;

#[cgp_provider(UnescrowTokenRegistrarComponent)]
impl<Chain: Async, Counterparty: Async>
    UnescrowTokenRegistrar<MockChain<Chain, Counterparty>, MockChain<Counterparty, Chain>>
    for MockChainComponents
{
    async fn register_unescrow_token(
        chain: &mut MockChain<Chain, Counterparty>,
        src_channel_id: &Tagged<Counterparty, Chain, MockChannelId>,
        dst_channel_id: &Tagged<Chain, Counterparty, MockChannelId>,
        src_app_id: &Tagged<Counterparty, Chain, MockAppId>,
        dst_app_id: &Tagged<Chain, Counterparty, MockAppId>,
        amount: &MockAmount<Chain, Counterparty>,
    ) -> Result<(), String> {
        let state = chain.pending_state.mock_chain_state_mut();

        let quantity = state
            .escrow_balances
            .entry((
                dst_channel_id.clone(),
                src_channel_id.clone(),
                dst_app_id.clone(),
                src_app_id.clone(),
                amount.denom.clone(),
            ))
            .or_default();

        quantity.value = quantity
            .value
            .checked_sub(amount.quantity.value)
            .ok_or_else(|| "unescrow amount exceeded".to_owned())?;

        Ok(())
    }
}

#[cgp_provider(EscrowTokenRegistrarComponent)]
impl<Chain: Async, Counterparty: Async>
    EscrowTokenRegistrar<MockChain<Chain, Counterparty>, MockChain<Counterparty, Chain>>
    for MockChainComponents
{
    async fn register_escrowed_token(
        chain: &mut MockChain<Chain, Counterparty>,
        src_channel_id: &Tagged<Chain, Counterparty, MockChannelId>,
        dst_channel_id: &Tagged<Counterparty, Chain, MockChannelId>,
        src_app_id: &Tagged<Chain, Counterparty, MockAppId>,
        dst_app_id: &Tagged<Counterparty, Chain, MockAppId>,
        amount: &MockAmount<Chain, Counterparty>,
    ) -> Result<(), String> {
        let state = chain.pending_state.mock_chain_state_mut();

        let quantity = state
            .escrow_balances
            .entry((
                src_channel_id.clone(),
                dst_channel_id.clone(),
                src_app_id.clone(),
                dst_app_id.clone(),
                amount.denom.clone(),
            ))
            .or_default();

        quantity.value = quantity
            .value
            .checked_add(amount.quantity.value)
            .ok_or_else(|| "escrow amount overflow".to_owned())?;

        Ok(())
    }
}
