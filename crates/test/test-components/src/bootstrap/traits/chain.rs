use cgp::prelude::*;

use crate::driver::traits::HasChainDriverType;

#[cgp_component {
    provider: ChainBootstrapper,
    context: Bootstrap,
}]
#[async_trait]
pub trait CanBootstrapChain: HasChainDriverType + HasAsyncErrorType {
    async fn bootstrap_chain(
        &self,
        chain_id_prefix: &str,
    ) -> Result<Self::ChainDriver, Self::Error>;
}
