use alloc::string::String;

use cgp::prelude::*;

use crate::relayer_mock::base::types::runtime::MockRuntimeContext;
use crate::relayer_mock::contexts::chain::MockChainContext;

#[derive(HasField)]
pub struct MockRelayContext {
    pub src_chain: MockChainContext,
    pub dst_chain: MockChainContext,
    pub src_to_dst_client: String,
    pub dst_to_src_client: String,
    pub runtime: MockRuntimeContext,
}

impl MockRelayContext {
    pub fn new(
        src_chain: MockChainContext,
        dst_chain: MockChainContext,
        src_to_dst_client: String,
        dst_to_src_client: String,
        runtime: MockRuntimeContext,
    ) -> Self {
        Self {
            src_chain,
            dst_chain,
            src_to_dst_client,
            dst_to_src_client,
            runtime,
        }
    }

    pub fn src_to_dst_client(&self) -> &String {
        &self.src_to_dst_client
    }

    pub fn dst_to_src_client(&self) -> &String {
        &self.dst_to_src_client
    }
}
