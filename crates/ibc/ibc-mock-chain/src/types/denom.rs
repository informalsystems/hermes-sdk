use core::fmt::{Debug, Display};

use alloc::boxed::Box;
use cgp::core::Async;
use hermes_chain_type_components::traits::types::denom::ProvideDenomType;

use crate::contexts::chain::MockChain;
use crate::types::app_id::MockAppId;
use crate::types::channel_id::MockChannelId;
use crate::types::tagged::Tagged;

pub enum MockDenom<Chain, Counterparty> {
    Native(Tagged<Chain, Counterparty, MockNativeDenom>),
    Ibc(MockIbcDenom<Chain, Counterparty>),
}

#[derive(Debug, Clone)]
pub enum MockNativeDenom {
    CoinA,
    CoinB,
    CoinC,
}

pub struct MockIbcDenom<Chain, Counterparty> {
    pub src_channel_id: Tagged<Counterparty, Chain, MockChannelId>,
    pub dst_channel_id: Tagged<Chain, Counterparty, MockChannelId>,
    pub src_app_id: Tagged<Counterparty, Chain, MockAppId>,
    pub dst_app_id: Tagged<Chain, Counterparty, MockAppId>,
    pub src_denom: Box<MockDenom<Counterparty, Chain>>,
}

pub struct UseMockDenomType;

impl<Chain: Async, Counterparty: Async> ProvideDenomType<MockChain<Chain, Counterparty>>
    for UseMockDenomType
{
    type Denom = MockDenom<Chain, Counterparty>;
}

impl<Chain, Counterparty> Debug for MockDenom<Chain, Counterparty> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Native(denom) => f.debug_tuple("Native").field(denom).finish(),
            Self::Ibc(denom) => f.debug_tuple("Ibc").field(denom).finish(),
        }
    }
}

impl<Chain, Counterparty> Debug for MockIbcDenom<Chain, Counterparty> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("MockIbcDenom")
            .field("src_channel_id", &self.src_channel_id)
            .field("dst_channel_id", &self.dst_channel_id)
            .field("src_app_id", &self.src_app_id)
            .field("dst_app_id", &self.dst_app_id)
            .field("src_denom", &self.src_denom)
            .finish()
    }
}

impl<Chain, Counterparty> Display for MockDenom<Chain, Counterparty> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl<Chain, Counterparty> Clone for MockDenom<Chain, Counterparty> {
    fn clone(&self) -> Self {
        match self {
            Self::Native(denom) => Self::Native(denom.clone()),
            Self::Ibc(denom) => Self::Ibc(denom.clone()),
        }
    }
}

impl<Chain, Counterparty> Clone for MockIbcDenom<Chain, Counterparty> {
    fn clone(&self) -> Self {
        Self {
            src_channel_id: self.src_channel_id.clone(),
            dst_channel_id: self.dst_channel_id.clone(),
            src_app_id: self.src_app_id.clone(),
            dst_app_id: self.dst_app_id.clone(),
            src_denom: self.src_denom.clone(),
        }
    }
}
