use alloc::boxed::Box;
use core::cmp::Ordering;
use core::fmt::{Debug, Display};

use cgp::prelude::*;
use hermes_chain_type_components::traits::{DenomTypeComponent, ProvideDenomType};

use crate::components::chain::MockChainComponents;
use crate::contexts::chain::MockChain;
use crate::types::app_id::MockAppId;
use crate::types::channel_id::MockChannelId;
use crate::types::tagged::Tagged;

pub enum MockDenom<Chain, Counterparty> {
    Native(Tagged<Chain, Counterparty, MockNativeDenom>),
    Ibc(MockIbcDenom<Chain, Counterparty>),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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

#[cgp_provider(DenomTypeComponent)]
impl<Chain: Async, Counterparty: Async> ProvideDenomType<MockChain<Chain, Counterparty>>
    for MockChainComponents
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

impl<Chain, Counterparty> PartialEq for MockIbcDenom<Chain, Counterparty> {
    fn eq(&self, other: &Self) -> bool {
        self.src_channel_id == other.src_channel_id
            && self.dst_channel_id == other.dst_channel_id
            && self.src_app_id == other.src_app_id
            && self.dst_app_id == other.dst_app_id
            && self.src_denom == other.src_denom
    }
}

impl<Chain, Counterparty> PartialEq for MockDenom<Chain, Counterparty> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Native(l0), Self::Native(r0)) => l0 == r0,
            (Self::Ibc(l0), Self::Ibc(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl<Chain, Counterparty> Eq for MockIbcDenom<Chain, Counterparty> {}

impl<Chain, Counterparty> Eq for MockDenom<Chain, Counterparty> {}

impl<Chain, Counterparty> PartialOrd for MockIbcDenom<Chain, Counterparty> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<Chain, Counterparty> PartialOrd for MockDenom<Chain, Counterparty> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<Chain, Counterparty> Ord for MockIbcDenom<Chain, Counterparty> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.src_channel_id.cmp(&other.src_channel_id) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        match self.dst_channel_id.cmp(&other.dst_channel_id) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        match self.src_app_id.cmp(&other.src_app_id) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        match self.dst_app_id.cmp(&other.dst_app_id) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        self.src_denom.cmp(&other.src_denom)
    }
}

impl<Chain, Counterparty> Ord for MockDenom<Chain, Counterparty> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (MockDenom::Native(target), MockDenom::Native(other)) => target.cmp(other),
            (MockDenom::Native(_), MockDenom::Ibc(_)) => Ordering::Less,
            (MockDenom::Ibc(_), MockDenom::Native(_)) => Ordering::Greater,
            (MockDenom::Ibc(target), MockDenom::Ibc(other)) => target.cmp(other),
        }
    }
}
