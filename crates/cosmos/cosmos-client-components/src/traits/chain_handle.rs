use cgp_core::prelude::*;
use cgp_core::CanRaiseError;
use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer::error::Error as RelayerError;

#[async_trait]
pub trait HasBlockingChainHandle: CanRaiseError<RelayerError> {
    type ChainHandle: ChainHandle;

    async fn with_blocking_chain_handle<R>(
        &self,
        cont: impl FnOnce(Self::ChainHandle) -> Result<R, Self::Error> + Send + 'static,
    ) -> Result<R, Self::Error>
    where
        R: Send + 'static;
}
