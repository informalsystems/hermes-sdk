use alloc::format;
use core::fmt::Display;
use core::marker::PhantomData;

use cgp::core::error::{CanRaiseAsyncError, ErrorOf};
use cgp::core::Async;

use crate::bootstrap::traits::chain::CanBootstrapChain;
use crate::driver::traits::types::chain_driver_at::ChainDriverTypeAt;
use crate::setup::traits::bootstrap_at::HasBootstrapAt;
use crate::setup::traits::chain::ChainSetup;

pub struct SetupChainWithBootstrap;

impl<Setup, I> ChainSetup<Setup, I> for SetupChainWithBootstrap
where
    Setup: HasBootstrapAt<I> + CanRaiseAsyncError<ErrorOf<Setup::Bootstrap>>,
    Setup::Bootstrap: CanBootstrapChain,
    I: Async + Default + Display,
{
    async fn setup_chain(
        setup: &Setup,
        _index: PhantomData<I>,
    ) -> Result<ChainDriverTypeAt<Setup, I>, Setup::Error> {
        let bootstrap = setup.chain_bootstrap(PhantomData);

        let chain_name = format!("chain-{}", I::default());

        let chain = bootstrap
            .bootstrap_chain(&chain_name)
            .await
            .map_err(Setup::raise_error)?;

        Ok(chain)
    }
}
