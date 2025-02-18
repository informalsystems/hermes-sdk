use alloc::format;
use core::marker::PhantomData;

use cgp::core::error::{CanRaiseAsyncError, ErrorOf};
use cgp::core::field::Index;
use cgp::prelude::*;

use crate::bootstrap::traits::chain::CanBootstrapChain;
use crate::driver::traits::types::chain_driver_at::ChainDriverTypeAt;
use crate::setup::traits::bootstrap_at::HasBootstrapAt;
use crate::setup::traits::chain::{ChainSetup, ChainSetupComponent};

pub struct SetupChainWithBootstrap;

#[cgp_provider(ChainSetupComponent)]
impl<Setup, const I: usize> ChainSetup<Setup, Index<I>> for SetupChainWithBootstrap
where
    Setup: HasBootstrapAt<Index<I>> + CanRaiseAsyncError<ErrorOf<Setup::Bootstrap>>,
    Setup::Bootstrap: CanBootstrapChain,
{
    async fn setup_chain(
        setup: &Setup,
        _index: PhantomData<Index<I>>,
    ) -> Result<ChainDriverTypeAt<Setup, Index<I>>, Setup::Error> {
        let bootstrap = setup.chain_bootstrap(PhantomData);

        let chain_name = format!("chain-{}", I);

        let chain = bootstrap
            .bootstrap_chain(&chain_name)
            .await
            .map_err(Setup::raise_error)?;

        Ok(chain)
    }
}
