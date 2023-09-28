use ibc_relayer_components::relay::impls::channel::open_init::CanRaiseMissingChannelInitEventError;
use ibc_relayer_components::relay::impls::channel::open_try::CanRaiseMissingChannelTryEventError;

use crate::one_for_all::traits::chain::OfaChainTypes;
use crate::one_for_all::traits::relay::OfaRelay;
use crate::one_for_all::types::relay::OfaRelayWrapper;

impl<Relay> CanRaiseMissingChannelInitEventError for OfaRelayWrapper<Relay>
where
    Relay: OfaRelay,
{
    fn missing_channel_init_event_error(&self) -> Relay::Error {
        self.relay.missing_channel_init_event_error()
    }
}

impl<Relay> CanRaiseMissingChannelTryEventError for OfaRelayWrapper<Relay>
where
    Relay: OfaRelay,
{
    fn missing_channel_try_event_error(
        &self,
        src_channel_id: &<Relay::SrcChain as OfaChainTypes>::ChannelId,
    ) -> Relay::Error {
        self.relay.missing_channel_try_event_error(src_channel_id)
    }
}
