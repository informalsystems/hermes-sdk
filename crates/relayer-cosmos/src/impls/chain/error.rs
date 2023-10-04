use cgp_core::{Async, CanRaiseError};
use ibc_relayer::error::Error as RelayerError;

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
