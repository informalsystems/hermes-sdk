use eyre::eyre;
use hermes_relayer_components::relay::impls::channel::open_init::CanRaiseMissingChannelInitEventError;
use hermes_relayer_components::relay::impls::channel::open_try::CanRaiseMissingChannelTryEventError;
use ibc_relayer_types::core::ics24_host::identifier::ChannelId;

use crate::contexts::relay::CosmosRelay;
use crate::types::error::Error;

impl CanRaiseMissingChannelInitEventError for CosmosRelay {
    fn missing_channel_init_event_error(&self) -> Error {
        eyre!("missing_channel_init_event_error").into()
    }
}

impl CanRaiseMissingChannelTryEventError for CosmosRelay {
    fn missing_channel_try_event_error(&self, src_channel_id: &ChannelId) -> Error {
        eyre!("missing_channel_try_event_error: {}", src_channel_id).into()
    }
}
