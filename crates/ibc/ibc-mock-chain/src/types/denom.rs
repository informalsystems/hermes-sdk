use core::fmt::{Debug, Display};

use alloc::boxed::Box;

use crate::types::app_id::MockAppId;
use crate::types::channel_id::MockChannelId;

#[derive(Debug, Clone)]
pub enum MockDenom {
    Native(MockNativeDenom),
    Ibc(MockIbcDenom),
}

#[derive(Debug, Clone)]
pub enum MockNativeDenom {
    CoinA,
    CoinB,
    CoinC,
}

#[derive(Debug, Clone)]
pub struct MockIbcDenom {
    pub src_channel_id: MockChannelId,
    pub dst_channel_id: MockChannelId,
    pub src_app_id: MockAppId,
    pub dst_app_id: MockAppId,
    pub src_denom: Box<MockDenom>,
}

impl Display for MockDenom {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(self, f)
    }
}
