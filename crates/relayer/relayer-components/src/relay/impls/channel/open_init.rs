use core::fmt::Debug;
use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_chain_components::traits::extract_data::CanExtractFromMessageResponse;

use crate::chain::traits::message_builders::channel_handshake::CanBuildChannelOpenInitMessage;
use crate::chain::traits::send_message::CanSendSingleMessage;
use crate::chain::traits::types::channel::HasInitChannelOptionsType;
use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::chain::traits::types::ibc_events::channel::HasChannelOpenInitEvent;
use crate::relay::traits::chains::HasRelayChains;
use crate::relay::traits::channel::open_init::{ChannelInitializer, ChannelInitializerComponent};

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
        + CanRaiseAsyncError<SrcChain::Error>,
    SrcChain: CanSendSingleMessage
        + HasInitChannelOptionsType<DstChain>
        + CanBuildChannelOpenInitMessage<DstChain>
        + HasChannelOpenInitEvent<DstChain>
        + CanExtractFromMessageResponse<SrcChain::ChannelOpenInitEvent>,
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

        Ok(src_channel_id.clone())
    }
}

impl<'a, Relay> Debug for MissingChannelInitEventError<'a, Relay> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "missing channel open init event")
    }
}
