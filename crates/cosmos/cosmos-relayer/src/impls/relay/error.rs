use eyre::eyre;
use hermes_relayer_components::relay::impls::channel::open_init::CanRaiseMissingChannelInitEventError;
use hermes_relayer_components::relay::impls::channel::open_try::CanRaiseMissingChannelTryEventError;
use hermes_relayer_components::relay::impls::connection::open_init::CanRaiseMissingConnectionInitEventError;
use hermes_relayer_components::relay::impls::connection::open_try::CanRaiseMissingConnectionTryEventError;
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, ConnectionId};

use crate::contexts::relay::CosmosRelay;
use crate::types::error::Error;

impl CanRaiseMissingConnectionInitEventError for CosmosRelay {
    fn missing_connection_init_event_error(&self) -> Error {
        eyre!("missing_connection_init_event_error").into()
    }
}

impl CanRaiseMissingConnectionTryEventError for CosmosRelay {
    fn missing_connection_try_event_error(&self, src_connection_id: &ConnectionId) -> Error {
        eyre!("missing_connection_try_event_error: {}", src_connection_id).into()
    }
}

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
