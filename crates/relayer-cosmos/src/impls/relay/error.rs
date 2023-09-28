use cgp_core::traits::Async;
use eyre::eyre;
use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer_all_in_one::one_for_all::types::chain::OfaChainWrapper;
use ibc_relayer_components::chain::traits::types::chain_id::HasChainId;
use ibc_relayer_components::relay::components::create_client::CanRaiseMissingCreateClientEventError;
use ibc_relayer_components::relay::impls::channel::open_init::CanRaiseMissingChannelInitEventError;
use ibc_relayer_components::relay::impls::channel::open_try::CanRaiseMissingChannelTryEventError;
use ibc_relayer_components::relay::impls::connection::open_init::CanRaiseMissingConnectionInitEventError;
use ibc_relayer_components::relay::impls::connection::open_try::CanRaiseMissingConnectionTryEventError;
use ibc_relayer_components::relay::traits::target::{DestinationTarget, SourceTarget};
use ibc_relayer_components_extra::relay::components::packet_relayers::retry::SupportsPacketRetry;
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, ConnectionId};

use crate::contexts::chain::CosmosChain;
use crate::contexts::relay::CosmosRelay;
use crate::types::error::{BaseError, Error};

impl<SrcChain, DstChain> SupportsPacketRetry for CosmosRelay<SrcChain, DstChain>
where
    SrcChain: Async,
    DstChain: Async,
{
    const MAX_RETRY: usize = 3;

    fn is_retryable_error(_: &Error) -> bool {
        false
    }

    fn max_retry_exceeded_error(e: Error) -> Error {
        e
    }
}

impl<SrcChain, DstChain> CanRaiseMissingCreateClientEventError<SourceTarget>
    for CosmosRelay<SrcChain, DstChain>
where
    SrcChain: ChainHandle,
    DstChain: ChainHandle,
{
    fn missing_create_client_event_error(
        src_chain: &OfaChainWrapper<CosmosChain<SrcChain>>,
        dst_chain: &OfaChainWrapper<CosmosChain<DstChain>>,
    ) -> Error {
        BaseError::generic(eyre!("missing CreateClient event when creating client from chain {} with counterparty chain {}",
            src_chain.chain_id(),
            dst_chain.chain_id(),
        )).into()
    }
}

impl<SrcChain, DstChain> CanRaiseMissingCreateClientEventError<DestinationTarget>
    for CosmosRelay<SrcChain, DstChain>
where
    SrcChain: ChainHandle,
    DstChain: ChainHandle,
{
    fn missing_create_client_event_error(
        dst_chain: &OfaChainWrapper<CosmosChain<DstChain>>,
        src_chain: &OfaChainWrapper<CosmosChain<SrcChain>>,
    ) -> Error {
        BaseError::generic(eyre!("missing CreateClient event when creating client from chain {} with counterparty chain {}",
            dst_chain.chain_id(),
            src_chain.chain_id(),
        )).into()
    }
}

impl<SrcChain, DstChain> CanRaiseMissingConnectionInitEventError for CosmosRelay<SrcChain, DstChain>
where
    SrcChain: ChainHandle,
    DstChain: ChainHandle,
{
    fn missing_connection_init_event_error(&self) -> Error {
        BaseError::generic(eyre!("missing_connection_init_event_error")).into()
    }
}

impl<SrcChain, DstChain> CanRaiseMissingConnectionTryEventError for CosmosRelay<SrcChain, DstChain>
where
    SrcChain: ChainHandle,
    DstChain: ChainHandle,
{
    fn missing_connection_try_event_error(&self, src_connection_id: &ConnectionId) -> Error {
        BaseError::generic(eyre!(
            "missing_connection_try_event_error: {}",
            src_connection_id
        ))
        .into()
    }
}

impl<SrcChain, DstChain> CanRaiseMissingChannelInitEventError for CosmosRelay<SrcChain, DstChain>
where
    SrcChain: ChainHandle,
    DstChain: ChainHandle,
{
    fn missing_channel_init_event_error(&self) -> Error {
        BaseError::generic(eyre!("missing_channel_init_event_error")).into()
    }
}

impl<SrcChain, DstChain> CanRaiseMissingChannelTryEventError for CosmosRelay<SrcChain, DstChain>
where
    SrcChain: ChainHandle,
    DstChain: ChainHandle,
{
    fn missing_channel_try_event_error(&self, src_channel_id: &ChannelId) -> Error {
        BaseError::generic(eyre!("missing_channel_try_event_error: {}", src_channel_id)).into()
    }
}
