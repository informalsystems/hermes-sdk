use eyre::eyre;
use hermes_cosmos_relayer::types::error::Error;
use hermes_relayer_components::relay::impls::connection::open_init::CanRaiseMissingConnectionInitEventError;

use crate::context::relay::SolomachineRelay;
use crate::traits::solomachine::Solomachine;

impl<Chain> CanRaiseMissingConnectionInitEventError for SolomachineRelay<Chain>
where
    Chain: Solomachine<Error = Error>,
{
    fn missing_connection_init_event_error(&self) -> Self::Error {
        eyre!("missing ConnectionOpenInit event").into()
    }
}
