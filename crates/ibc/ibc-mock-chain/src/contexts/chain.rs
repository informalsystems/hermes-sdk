use alloc::string::String;
use cgp::prelude::*;
use hermes_chain_type_components::traits::types::ibc::channel_id::HasChannelIdType;
use hermes_ibc_components::traits::types::app_id::HasAppIdType;

use crate::components::chain::MockChainComponents;
use crate::types::app_id::MockAppId;
use crate::types::channel_id::MockChannelId;

pub struct MockChain;

impl HasComponents for MockChain {
    type Components = MockChainComponents;
}

pub trait CanUseMockChain:
    HasErrorType<Error = String>
    + HasAppIdType<MockChain, AppId = MockAppId>
    + HasChannelIdType<MockChain, ChannelId = MockChannelId>
{
}

impl CanUseMockChain for MockChain {}
