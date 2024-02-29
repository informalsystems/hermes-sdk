use eyre::eyre;
use hermes_relayer_components::relay::impls::connection::open_init::CanRaiseMissingConnectionInitEventError;

use crate::context::relay::SolomachineRelay;
use crate::traits::solomachine::Solomachine;
use crate::types::error::{BaseError, Error};

impl<Chain> CanRaiseMissingConnectionInitEventError for SolomachineRelay<Chain>
where
    Chain: Solomachine<Error = Error>,
{
    fn missing_connection_init_event_error(&self) -> Self::Error {
        BaseError::generic(eyre!("missing ConnectionOpenInit event")).into()
    }
}
