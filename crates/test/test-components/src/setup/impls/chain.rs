use alloc::format;

use cgp_core::CanRaiseError;
use hermes_relayer_components::runtime::types::aliases::ErrorOf;

use crate::bootstrap::traits::chain::CanBootstrapChain;
use crate::driver::traits::types::chain_driver_at::ChainDriverTypeAt;
use crate::setup::traits::bootstrap_at::HasBootstrapAt;
use crate::setup::traits::chain::ChainSetup;
use crate::types::index::Index;

pub struct SetupChainWithBootstrap;

impl<Setup, const I: usize> ChainSetup<Setup, I> for SetupChainWithBootstrap
where
    Setup: HasBootstrapAt<I> + CanRaiseError<ErrorOf<Setup::Bootstrap>>,
    Setup::Bootstrap: CanBootstrapChain,
{
    async fn setup_chain(
        setup: &Setup,
        _index: Index<I>,
    ) -> Result<ChainDriverTypeAt<Setup, I>, Setup::Error> {
        let bootstrap = setup.chain_bootstrap(Index::<I>);

        let chain_name = format!("chain-{}", I);

        let chain = bootstrap
            .bootstrap_chain(&chain_name)
            .await
            .map_err(Setup::raise_error)?;

        Ok(chain)
    }
}
