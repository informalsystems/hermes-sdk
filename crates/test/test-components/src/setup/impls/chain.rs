use alloc::format;
use cgp_core::CanRaiseError;

use crate::bootstrap::traits::chain::CanBootstrapChain;
use crate::driver::traits::types::chain_at::ChainTypeAt;
use crate::setup::traits::bootstrap_at::HasBootstrapAt;
use crate::setup::traits::chain::ChainSetup;
use crate::types::error::ErrorOf;
use crate::types::index::Index;

pub struct SetupChain;

impl<Setup, const I: usize> ChainSetup<Setup, I> for SetupChain
where
    Setup: HasBootstrapAt<I> + CanRaiseError<ErrorOf<Setup::Bootstrap>>,
    Setup::Bootstrap: CanBootstrapChain,
{
    async fn setup_chain(
        setup: &Setup,
        _index: Index<I>,
    ) -> Result<ChainTypeAt<Setup, I>, Setup::Error> {
        let bootstrap = setup.chain_bootstrap(Index::<I>);

        let chain_name = format!("chain-{}", I);

        let chain = bootstrap
            .bootstrap_chain(&chain_name)
            .await
            .map_err(Setup::raise_error)?;

        Ok(chain)
    }
}
