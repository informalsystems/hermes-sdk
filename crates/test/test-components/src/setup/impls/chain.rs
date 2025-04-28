use alloc::format;
use core::marker::PhantomData;

use cgp::core::error::{CanRaiseAsyncError, ErrorOf};
use cgp::core::field::Index;
use hermes_prelude::*;

use crate::bootstrap::traits::CanBootstrapChain;
use crate::driver::traits::{ChainDriverAt, HasChainDriverTypeAt};
use crate::setup::traits::{ChainSetup, ChainSetupComponent, HasBootstrapAt};

#[cgp_new_provider(ChainSetupComponent)]
impl<Setup, ChainDriver, const I: usize> ChainSetup<Setup, Index<I>> for SetupChainWithBootstrap
where
    Setup: HasBootstrapAt<Index<I>>
        + HasChainDriverTypeAt<Index<I>, ChainDriver = ChainDriver>
        + CanRaiseAsyncError<ErrorOf<Setup::Bootstrap>>,
    Setup::Bootstrap: CanBootstrapChain<ChainDriver = ChainDriver>,
{
    async fn setup_chain(
        setup: &Setup,
        _index: PhantomData<Index<I>>,
    ) -> Result<ChainDriverAt<Setup, Index<I>>, Setup::Error> {
        let bootstrap = setup.chain_bootstrap(PhantomData);

        let chain_name = format!("chain-{}", I);

        let chain = bootstrap
            .bootstrap_chain(&chain_name)
            .await
            .map_err(Setup::raise_error)?;

        Ok(chain)
    }
}
