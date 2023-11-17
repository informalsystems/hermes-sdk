use alloc::boxed::Box;

use cgp_core::prelude::*;

#[derive_component(ChainBootstrapperComponent, ChainBootstrapper<Bootstrap>)]
#[async_trait]
pub trait CanBootstrapChain<Chain>: HasErrorType {
    async fn bootstrap_chain(&self, chain_id_prefix: &str) -> Result<Chain, Self::Error>;
}
