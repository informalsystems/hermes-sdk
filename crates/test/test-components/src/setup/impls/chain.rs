use alloc::format;

use cgp::core::error::{CanRaiseError, ErrorOf};
use hermes_relayer_components::multi::types::index::Index;

use crate::bootstrap::traits::chain::CanBootstrapChain;
use crate::driver::traits::types::chain_driver_at::ChainDriverTypeAt;
use crate::setup::traits::bootstrap_at::HasBootstrapAt;
use crate::setup::traits::chain::ChainSetup;

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
