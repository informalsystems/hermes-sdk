use alloc::string::String;
use cgp::prelude::*;

use crate::components::chain::MockChainComponents;

pub struct MockChain;

impl HasComponents for MockChain {
    type Components = MockChainComponents;
}

pub trait CanUseMockChain: HasErrorType<Error = String> {}

impl CanUseMockChain for MockChain {}
