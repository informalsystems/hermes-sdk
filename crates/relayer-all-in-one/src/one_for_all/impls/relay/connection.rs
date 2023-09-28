use ibc_relayer_components::relay::impls::connection::open_init::CanRaiseMissingConnectionInitEventError;
use ibc_relayer_components::relay::impls::connection::open_try::CanRaiseMissingConnectionTryEventError;

use crate::one_for_all::traits::chain::OfaChainTypes;
use crate::one_for_all::traits::relay::OfaRelay;
use crate::one_for_all::types::relay::OfaRelayWrapper;

impl<Relay> CanRaiseMissingConnectionInitEventError for OfaRelayWrapper<Relay>
where
    Relay: OfaRelay,
{
    fn missing_connection_init_event_error(&self) -> Relay::Error {
        self.relay.missing_connection_init_event_error()
    }
}

impl<Relay> CanRaiseMissingConnectionTryEventError for OfaRelayWrapper<Relay>
where
    Relay: OfaRelay,
{
    fn missing_connection_try_event_error(
        &self,
        src_connection_id: &<Relay::SrcChain as OfaChainTypes>::ConnectionId,
    ) -> Relay::Error {
        self.relay
            .missing_connection_try_event_error(src_connection_id)
    }
}
