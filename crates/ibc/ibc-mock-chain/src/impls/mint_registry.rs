use alloc::boxed::Box;
use alloc::string::String;

use cgp::prelude::*;
use hermes_ibc_token_transfer_components::traits::mint_registry::lookup_incoming::{
    IncomingMintedTokenQuerier, IncomingMintedTokenQuerierComponent,
};
use hermes_ibc_token_transfer_components::traits::mint_registry::lookup_outgoing::{
    OutgoingBurnTokenQuerier, OutgoingBurnTokenQuerierComponent,
};
use hermes_ibc_token_transfer_components::traits::mint_registry::register::{
    MintedTokenRegistrar, MintedTokenRegistrarComponent,
};
use hermes_ibc_token_transfer_components::traits::token::create::{
    CanCreateToken, TokenCreator, TokenCreatorComponent,
};

use crate::components::chain::MockChainComponents;
use crate::contexts::chain::MockChain;
use crate::types::app_id::MockAppId;
use crate::types::channel_id::MockChannelId;
use crate::types::denom::{MockDenom, MockIbcDenom};
use crate::types::tagged::Tagged;

#[cgp_provider(IncomingMintedTokenQuerierComponent)]
impl<Chain: Async, Counterparty: Async>
    IncomingMintedTokenQuerier<MockChain<Chain, Counterparty>, MockChain<Counterparty, Chain>>
    for MockChainComponents
{
    async fn lookup_incoming_minted_token(
        chain: &MockChain<Chain, Counterparty>,
        src_channel_id: &Tagged<Counterparty, Chain, MockChannelId>,
        dst_channel_id: &Tagged<Chain, Counterparty, MockChannelId>,
        src_app_id: &Tagged<Counterparty, Chain, MockAppId>,
        dst_app_id: &Tagged<Chain, Counterparty, MockAppId>,
        src_denom: &MockDenom<Counterparty, Chain>,
    ) -> Result<Option<MockDenom<Chain, Counterparty>>, String> {
        let dst_denom = chain
            .create_token(
                src_channel_id,
                dst_channel_id,
                src_app_id,
                dst_app_id,
                src_denom,
            )
            .await?;

        Ok(Some(dst_denom))
    }
}

#[cgp_provider(TokenCreatorComponent)]
impl<Chain: Async, Counterparty: Async>
    TokenCreator<MockChain<Chain, Counterparty>, MockChain<Counterparty, Chain>>
    for MockChainComponents
{
    async fn create_token(
        _chain: &MockChain<Chain, Counterparty>,
        src_channel_id: &Tagged<Counterparty, Chain, MockChannelId>,
        dst_channel_id: &Tagged<Chain, Counterparty, MockChannelId>,
        src_app_id: &Tagged<Counterparty, Chain, MockAppId>,
        dst_app_id: &Tagged<Chain, Counterparty, MockAppId>,
        src_denom: &MockDenom<Counterparty, Chain>,
    ) -> Result<MockDenom<Chain, Counterparty>, String> {
        Ok(MockDenom::Ibc(MockIbcDenom {
            src_channel_id: src_channel_id.clone(),
            dst_channel_id: dst_channel_id.clone(),
            src_app_id: src_app_id.clone(),
            dst_app_id: dst_app_id.clone(),
            src_denom: Box::new(src_denom.clone()),
        }))
    }
}

#[cgp_provider(MintedTokenRegistrarComponent)]
impl<Chain: Async, Counterparty: Async>
    MintedTokenRegistrar<MockChain<Chain, Counterparty>, MockChain<Counterparty, Chain>>
    for MockChainComponents
{
    async fn register_minted_token(
        _chain: &MockChain<Chain, Counterparty>,
        src_channel_id: &Tagged<Counterparty, Chain, MockChannelId>,
        dst_channel_id: &Tagged<Chain, Counterparty, MockChannelId>,
        src_app_id: &Tagged<Counterparty, Chain, MockAppId>,
        dst_app_id: &Tagged<Chain, Counterparty, MockAppId>,
        src_denom: &MockDenom<Counterparty, Chain>,
        dst_denom: &MockDenom<Chain, Counterparty>,
    ) -> Result<(), String> {
        match dst_denom {
            MockDenom::Ibc(denom) => {
                if &denom.src_channel_id != src_channel_id
                    || &denom.dst_channel_id != dst_channel_id
                    || &denom.src_app_id != src_app_id
                    || &denom.dst_app_id != dst_app_id
                    || denom.src_denom.as_ref() != src_denom
                {
                    return Err("dst_denom must match generated IBC denom".into());
                }
            }
            MockDenom::Native(_denom) => return Err("expect dst_denom to be an IBC denom".into()),
        }

        Ok(())
    }
}

#[cgp_provider(OutgoingBurnTokenQuerierComponent)]
impl<Chain: Async, Counterparty: Async>
    OutgoingBurnTokenQuerier<MockChain<Chain, Counterparty>, MockChain<Counterparty, Chain>>
    for MockChainComponents
{
    async fn lookup_outgoing_burn_token(
        _chain: &MockChain<Chain, Counterparty>,
        src_channel_id: &Tagged<Chain, Counterparty, MockChannelId>,
        dst_channel_id: &Tagged<Counterparty, Chain, MockChannelId>,
        src_app_id: &Tagged<Chain, Counterparty, MockAppId>,
        dst_app_id: &Tagged<Counterparty, Chain, MockAppId>,
        src_denom: &MockDenom<Chain, Counterparty>,
    ) -> Result<Option<MockDenom<Counterparty, Chain>>, String> {
        match src_denom {
            MockDenom::Ibc(denom) => {
                if &denom.src_channel_id == dst_channel_id
                    && &denom.dst_channel_id == src_channel_id
                    && &denom.src_app_id == dst_app_id
                    && &denom.dst_app_id == src_app_id
                {
                    Ok(Some(denom.src_denom.as_ref().clone()))
                } else {
                    Ok(None)
                }
            }
            MockDenom::Native(_denom) => Ok(None),
        }
    }
}
