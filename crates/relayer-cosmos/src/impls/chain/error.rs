use cgp_core::{Async, ErrorRaiser};
use ibc_relayer::error::Error as RelayerError;
use ibc_relayer::supervisor::Error as SupervisorError;

use crate::contexts::chain::CosmosChain;
use crate::impls::chain::component::CosmosChainComponents;
use crate::types::error::{BaseError, Error};

impl<Chain> ErrorRaiser<CosmosChain<Chain>, RelayerError> for CosmosChainComponents
where
    Chain: Async,
{
    fn raise_error(err: RelayerError) -> Error {
        BaseError::relayer(err).into()
    }
}

impl<Chain> ErrorRaiser<CosmosChain<Chain>, SupervisorError> for CosmosChainComponents
where
    Chain: Async,
{
    fn raise_error(err: SupervisorError) -> Error {
        BaseError::supervisor(err).into()
    }
}

impl<Chain> ErrorRaiser<CosmosChain<Chain>, eyre::Report> for CosmosChainComponents
where
    Chain: Async,
{
    fn raise_error(err: eyre::Report) -> Error {
        BaseError::generic(err).into()
    }
}
