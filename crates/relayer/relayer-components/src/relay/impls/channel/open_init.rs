use alloc::format;
use core::fmt::Debug;
use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_chain_components::traits::{CanExtractFromMessageResponse, HasChainId};
use hermes_logging_components::traits::CanLog;
use hermes_logging_components::types::LevelInfo;

use crate::chain::traits::{
    CanBuildChannelOpenInitMessage, CanSendSingleMessage, HasChannelOpenInitEvent,
    HasIbcChainTypes, HasInitChannelOptionsType,
};
use crate::relay::traits::{ChannelInitializer, ChannelInitializerComponent, HasRelayChains};

/**
   A base implementation for [`ChannelInitializer`] which submits a
   `ChannelOpenInit` message to the source chain.

   This implements the `ChanOpenInit` step in the IBC channel handshake protocol.
*/
pub struct InitializeChannel;

pub struct MissingChannelInitEventError<'a, Relay> {
    pub relay: &'a Relay,
}

#[cgp_provider(ChannelInitializerComponent)]
impl<Relay, SrcChain, DstChain> ChannelInitializer<Relay> for InitializeChannel
where
    Relay: HasRelayChains<SrcChain = SrcChain, DstChain = DstChain>
        + for<'a> CanRaiseAsyncError<MissingChannelInitEventError<'a, Relay>>
        + CanLog<LevelInfo>
        + CanRaiseAsyncError<SrcChain::Error>,
    SrcChain: CanSendSingleMessage
        + HasInitChannelOptionsType<DstChain>
        + CanBuildChannelOpenInitMessage<DstChain>
        + HasChannelOpenInitEvent<DstChain>
        + CanExtractFromMessageResponse<SrcChain::ChannelOpenInitEvent>
        + HasChainId,
    DstChain: HasIbcChainTypes<SrcChain> + HasAsyncErrorType,
    SrcChain::ChannelId: Clone,
{
    async fn init_channel(
        relay: &Relay,
        src_port_id: &SrcChain::PortId,
        dst_port_id: &DstChain::PortId,
        init_channel_options: &SrcChain::InitChannelOptions,
    ) -> Result<SrcChain::ChannelId, Relay::Error> {
        let src_chain = relay.src_chain();

        relay
            .log(
                &format!(
                    "Starting ICS04 ChannelOpenInit on chain `{}`",
                    src_chain.chain_id()
                ),
                &LevelInfo,
            )
            .await;

        let src_message = src_chain
            .build_channel_open_init_message(src_port_id, dst_port_id, init_channel_options)
            .await
            .map_err(Relay::raise_error)?;

        let response = src_chain
            .send_message(src_message)
            .await
            .map_err(Relay::raise_error)?;

        let open_init_event = src_chain
            .try_extract_from_message_response(PhantomData, &response)
            .ok_or_else(|| Relay::raise_error(MissingChannelInitEventError { relay }))?;

        let src_channel_id = SrcChain::channel_open_init_event_channel_id(&open_init_event);

        relay
            .log(
                &format!(
                    "Successfully completed ICS04 ChannelOpenInit on chain {} with ChannelId `{src_channel_id}` and PortId `{src_port_id}`",
                    src_chain.chain_id()
                ),
                &LevelInfo,
            )
            .await;

        Ok(src_channel_id.clone())
    }
}

impl<'a, Relay> Debug for MissingChannelInitEventError<'a, Relay> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "missing channel open init event")
    }
}
