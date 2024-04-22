use eyre::eyre;
use hermes_relayer_components::relay::impls::channel::open_init::CanRaiseMissingChannelInitEventError;

use crate::contexts::relay::CosmosRelay;
use crate::types::error::Error;

impl CanRaiseMissingChannelInitEventError for CosmosRelay {
    fn missing_channel_init_event_error(&self) -> Error {
        eyre!("missing_channel_init_event_error").into()
    }
}
