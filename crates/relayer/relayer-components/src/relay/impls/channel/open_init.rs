use core::fmt::Debug;

use cgp_core::CanRaiseError;

use crate::chain::traits::message_builders::channel_handshake::CanBuildChannelHandshakeMessages;
use crate::chain::traits::payload_builders::channel_handshake::CanBuildChannelHandshakePayloads;
use crate::chain::traits::send_message::CanSendSingleMessage;
use crate::chain::traits::types::channel::HasInitChannelOptionsType;
use crate::chain::traits::types::ibc_events::channel::HasChannelOpenInitEvent;
use crate::relay::traits::chains::{CanRaiseRelayChainErrors, HasRelayChains};
use crate::relay::traits::channel::open_init::ChannelInitializer;

/**
   A base implementation for [`ChannelInitializer`] which submits a
   `ChannelOpenInit` message to the source chain.

   This implements the `ChanOpenInit` step in the IBC channel handshake protocol.
*/

pub struct InitializeChannel;

pub struct MissingChannelInitEventError<'a, Relay> {
    pub relay: &'a Relay,
}

impl<Relay, SrcChain, DstChain> ChannelInitializer<Relay> for InitializeChannel
where
    Relay: HasRelayChains<SrcChain = SrcChain, DstChain = DstChain>
        + for<'a> CanRaiseError<MissingChannelInitEventError<'a, Relay>>
        + CanRaiseRelayChainErrors,
    SrcChain: CanSendSingleMessage
        + HasInitChannelOptionsType<DstChain>
        + CanBuildChannelHandshakeMessages<DstChain>
        + HasChannelOpenInitEvent<DstChain>,
    DstChain: CanBuildChannelHandshakePayloads<SrcChain>,
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

        let events = src_chain
            .send_message(src_message)
            .await
            .map_err(Relay::raise_error)?;

        let open_init_event = events
            .into_iter()
            .find_map(|event| SrcChain::try_extract_channel_open_init_event(event))
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
