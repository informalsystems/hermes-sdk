use alloc::boxed::Box;

use cgp_core::prelude::*;

#[derive_component(ChainBootstrapperComponent, ChainBootstrapper<Bootstrap>)]
#[async_trait]
pub trait CanBootstrapChain<Chain, Override>: HasErrorType {
    async fn bootstrap_chain(&self) -> Result<Chain, Self::Error>;
}
