use alloc::sync::Arc;
use std::fmt::Debug;

use basecoin::store::context::ProvableStore;
use basecoin::store::impls::RevertibleStore;
use hermes_runtime::types::runtime::HermesRuntime;
use ibc::core::host::types::identifiers::ChainId;
use tendermint_testgen::Validator;
use tokio::runtime::Runtime as TokioRuntime;

use crate::contexts::basecoin::MockBasecoin;
use crate::contexts::chain::MockCosmosContext;
use crate::contexts::relay::MockCosmosRelay;
use crate::traits::endpoint::BasecoinEndpoint;

pub struct MockCosmosBuilder {
    pub runtime: HermesRuntime,
}

impl MockCosmosBuilder {
    pub fn new(runtime: TokioRuntime) -> Self {
        Self {
            runtime: HermesRuntime::new(Arc::new(runtime)),
        }
    }

    pub fn build_chain<S>(
        &mut self,
        chain_id: ChainId,
        validators: Vec<Validator>,
        store: RevertibleStore<S>,
    ) -> Arc<MockBasecoin<S>>
    where
        S: ProvableStore + Debug + Default,
    {
        let chain = Arc::new(MockBasecoin::new(
            self.runtime.clone(),
            chain_id,
            validators,
            store,
        ));

        chain.run();

        chain
    }

    pub fn build_relay<SrcChain: BasecoinEndpoint, DstChain: BasecoinEndpoint>(
        &self,
        src_chain: Arc<SrcChain>,
        dst_chain: Arc<DstChain>,
    ) -> Arc<MockCosmosRelay<SrcChain, DstChain>> {
        let src_chain_ctx = Arc::new(MockCosmosContext::new(self.runtime.clone(), src_chain));
        let dst_chain_ctx = Arc::new(MockCosmosContext::new(self.runtime.clone(), dst_chain));

        Arc::new(
            MockCosmosRelay::new(self.runtime.clone(), src_chain_ctx, dst_chain_ctx)
                .expect("failed to build relay"),
        )
    }
}
