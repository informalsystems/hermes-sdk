use alloc::string::String;
use cgp::prelude::*;
use hermes_ibc_components::traits::types::app_id::HasAppIdType;

use crate::components::chain::MockChainComponents;
use crate::types::app_id::MockAppId;

pub struct MockChain;

impl HasComponents for MockChain {
    type Components = MockChainComponents;
}

pub trait CanUseMockChain:
    HasErrorType<Error = String> + HasAppIdType<MockChain, AppId = MockAppId>
{
}

impl CanUseMockChain for MockChain {}
