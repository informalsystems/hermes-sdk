use core::fmt::Debug;
use core::marker::PhantomData;

use cgp::core::error::CanRaiseError;

use crate::chain::traits::message_builders::channel_handshake::CanBuildChannelOpenTryMessage;
use crate::chain::traits::payload_builders::channel_handshake::CanBuildChannelOpenTryPayload;
use crate::chain::traits::queries::chain_status::CanQueryChainHeight;
use crate::chain::traits::queries::client_state::CanQueryClientStateWithLatestHeight;
use crate::chain::traits::types::ibc_events::channel::HasChannelOpenTryEvent;
use crate::chain::types::aliases::ChannelIdOf;
use crate::relay::traits::chains::{CanRaiseRelayChainErrors, HasRelayChains, HasRelayClientIds};
use crate::relay::traits::channel::open_try::ChannelOpenTryRelayer;
use crate::relay::traits::ibc_message_sender::{CanSendSingleIbcMessage, MainSink};
use crate::relay::traits::target::{DestinationTarget, HasDestinationTargetChainTypes};
use crate::relay::types::aliases::{DstChannelId, DstPortId, SrcChannelId, SrcPortId};

/**
   A base implementation of [`ChannelOpenTryRelayer`] that relays a new channel
   at the source chain that is in `OPEN_INIT` state, and submits it as a
   `ChannelOpenTry` message to the destination chain.

   This implements the `ChanOpenTry` step of the IBC channel handshake protocol.

   Note that this implementation does not check that the channel exists on
   the destination chain. It also doesn't check that the channel end at the
   source chain is really in the `OPEN_INIT` state. This will be implemented as
   a separate wrapper component. (TODO)
*/
pub struct RelayChannelOpenTry;

pub struct MissingChannelTryEventError<'a, Relay>
where
    Relay: HasRelayChains,
{
    pub relay: &'a Relay,
    pub src_channel_id: &'a ChannelIdOf<Relay::SrcChain, Relay::DstChain>,
}

impl<Relay, SrcChain, DstChain> ChannelOpenTryRelayer<Relay> for RelayChannelOpenTry
where
    Relay: HasRelayChains<SrcChain = SrcChain, DstChain = DstChain>
        + HasDestinationTargetChainTypes
        + HasRelayClientIds
        + CanSendSingleIbcMessage<MainSink, DestinationTarget>
        + for<'a> CanRaiseError<MissingChannelTryEventError<'a, Relay>>
        + CanRaiseRelayChainErrors,
    SrcChain: CanQueryChainHeight + CanBuildChannelOpenTryPayload<DstChain>,
    DstChain: CanQueryClientStateWithLatestHeight<SrcChain>
        + CanBuildChannelOpenTryMessage<SrcChain>
        + HasChannelOpenTryEvent<SrcChain>,
    DstChain::ChannelId: Clone,
{
    async fn relay_channel_open_try(
        relay: &Relay,
        dst_port: &DstPortId<Relay>,
        src_port_id: &SrcPortId<Relay>,
        src_channel_id: &SrcChannelId<Relay>,
    ) -> Result<DstChannelId<Relay>, Relay::Error> {
        let src_chain = relay.src_chain();
        let dst_chain = relay.dst_chain();

        let src_proof_height = src_chain
            .query_chain_height()
            .await
            .map_err(Relay::raise_error)?;

        let src_client_state = dst_chain
            .query_client_state_with_latest_height(PhantomData, relay.dst_client_id())
            .await
            .map_err(Relay::raise_error)?;

        let open_try_payload = src_chain
            .build_channel_open_try_payload(
                &src_client_state,
                &src_proof_height,
                src_port_id,
                src_channel_id,
            )
            .await
            .map_err(Relay::raise_error)?;

        let open_try_message = dst_chain
            .build_channel_open_try_message(dst_port, src_port_id, src_channel_id, open_try_payload)
            .await
            .map_err(Relay::raise_error)?;

        let response = relay
            .send_message(DestinationTarget, open_try_message)
            .await?;

        let open_try_event =
            DstChain::try_extract_channel_open_try_event(&response).ok_or_else(|| {
                Relay::raise_error(MissingChannelTryEventError {
                    relay,
                    src_channel_id,
                })
            })?;

        let dst_channel_id = DstChain::channel_open_try_event_channel_id(&open_try_event);

        Ok(dst_channel_id.clone())
    }
}

impl<'a, Relay> Debug for MissingChannelTryEventError<'a, Relay>
where
    Relay: HasRelayChains,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("MissingChannelTryEventError")
            .field("src_channel_id", &self.src_channel_id)
            .finish()
    }
}
