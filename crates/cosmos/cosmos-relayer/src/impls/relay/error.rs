use eyre::eyre;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainId;
use hermes_relayer_components::relay::components::create_client::CanRaiseMissingCreateClientEventError;
use hermes_relayer_components::relay::impls::channel::open_init::CanRaiseMissingChannelInitEventError;
use hermes_relayer_components::relay::impls::channel::open_try::CanRaiseMissingChannelTryEventError;
use hermes_relayer_components::relay::impls::connection::open_init::CanRaiseMissingConnectionInitEventError;
use hermes_relayer_components::relay::impls::connection::open_try::CanRaiseMissingConnectionTryEventError;
use hermes_relayer_components::relay::traits::target::{DestinationTarget, SourceTarget};
use hermes_relayer_components_extra::relay::components::packet_relayers::retry::SupportsPacketRetry;

use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, ConnectionId};

use crate::contexts::chain::CosmosChain;
use crate::contexts::relay::CosmosRelay;
use crate::types::error::{BaseError, Error};

impl SupportsPacketRetry for CosmosRelay {
    const MAX_RETRY: usize = 3;

    fn is_retryable_error(_: &Error) -> bool {
        false
    }

    fn max_retry_exceeded_error(e: Error) -> Error {
        e
    }
}

impl CanRaiseMissingCreateClientEventError<SourceTarget> for CosmosRelay {
    fn missing_create_client_event_error(
        src_chain: &CosmosChain,
        dst_chain: &CosmosChain,
    ) -> Error {
        BaseError::generic(eyre!("missing CreateClient event when creating client from chain {} with counterparty chain {}",
            src_chain.chain_id(),
            dst_chain.chain_id(),
        )).into()
    }
}

impl CanRaiseMissingCreateClientEventError<DestinationTarget> for CosmosRelay {
    fn missing_create_client_event_error(
        dst_chain: &CosmosChain,
        src_chain: &CosmosChain,
    ) -> Error {
        BaseError::generic(eyre!("missing CreateClient event when creating client from chain {} with counterparty chain {}",
            dst_chain.chain_id(),
            src_chain.chain_id(),
        )).into()
    }
}

impl CanRaiseMissingConnectionInitEventError for CosmosRelay {
    fn missing_connection_init_event_error(&self) -> Error {
        BaseError::generic(eyre!("missing_connection_init_event_error")).into()
    }
}

impl CanRaiseMissingConnectionTryEventError for CosmosRelay {
    fn missing_connection_try_event_error(&self, src_connection_id: &ConnectionId) -> Error {
        BaseError::generic(eyre!(
            "missing_connection_try_event_error: {}",
            src_connection_id
        ))
        .into()
    }
}

impl CanRaiseMissingChannelInitEventError for CosmosRelay {
    fn missing_channel_init_event_error(&self) -> Error {
        BaseError::generic(eyre!("missing_channel_init_event_error")).into()
    }
}

impl CanRaiseMissingChannelTryEventError for CosmosRelay {
    fn missing_channel_try_event_error(&self, src_channel_id: &ChannelId) -> Error {
        BaseError::generic(eyre!("missing_channel_try_event_error: {}", src_channel_id)).into()
    }
}
