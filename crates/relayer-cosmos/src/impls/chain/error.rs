use cgp_core::{Async, CanRaiseError};
use ibc_relayer::error::Error as RelayerError;
use ibc_relayer::supervisor::Error as SupervisorError;

use crate::contexts::chain::CosmosChain;
use crate::types::error::{BaseError, Error};

impl<Chain> CanRaiseError<RelayerError> for CosmosChain<Chain>
where
    Chain: Async,
{
    fn raise_error(err: RelayerError) -> Error {
        BaseError::relayer(err).into()
    }
}

impl<Chain> CanRaiseError<SupervisorError> for CosmosChain<Chain>
where
    Chain: Async,
{
    fn raise_error(err: SupervisorError) -> Error {
        BaseError::supervisor(err).into()
    }
}

impl<Chain> CanRaiseError<eyre::Report> for CosmosChain<Chain>
where
    Chain: Async,
{
    fn raise_error(err: eyre::Report) -> Error {
        BaseError::generic(err).into()
    }
}
