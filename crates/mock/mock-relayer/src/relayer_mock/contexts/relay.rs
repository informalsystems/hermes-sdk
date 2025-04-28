use alloc::string::String;

use hermes_prelude::*;

use crate::relayer_mock::base::types::runtime::MockRuntimeContext;
use crate::relayer_mock::contexts::chain::MockChainContext;

#[cgp_context(MockRelayComponents)]
#[derive(HasField)]
pub struct MockRelayContext {
    pub src_chain: MockChainContext,
    pub dst_chain: MockChainContext,
    pub dst_client_id: String,
    pub src_client_id: String,
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
            dst_client_id: src_to_dst_client,
            src_client_id: dst_to_src_client,
            runtime,
        }
    }

    pub fn src_to_dst_client(&self) -> &String {
        &self.dst_client_id
    }

    pub fn dst_to_src_client(&self) -> &String {
        &self.src_client_id
    }
}
