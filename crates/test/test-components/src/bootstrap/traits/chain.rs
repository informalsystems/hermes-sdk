use cgp::prelude::*;

use crate::driver::traits::types::chain_driver::HasChainDriverType;

#[cgp_component {
  name: ChainBootstrapperComponent,
  provider: ChainBootstrapper,
  context: Bootstrap,
}]
#[async_trait]
pub trait CanBootstrapChain: HasChainDriverType + HasErrorType {
    async fn bootstrap_chain(
        &self,
        chain_id_prefix: &str,
    ) -> Result<Self::ChainDriver, Self::Error>;
}
