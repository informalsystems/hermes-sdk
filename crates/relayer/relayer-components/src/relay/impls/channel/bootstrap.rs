use hermes_prelude::*;

use crate::chain::traits::{HasIbcChainTypes, HasInitChannelOptionsType, InitChannelOptions};
use crate::relay::traits::{CanInitChannel, CanRelayChannelOpenHandshake, HasRelayChains};
use crate::relay::types::{DstChannelId, DstPortId, SrcChannelId, SrcPortId};

/**
   This is an autotrait implementation by the relay context to allow bootstrapping
   of new IBC channels as initiated by the relayer.

   This can be used by the users of the relayer to create new channels. It can
   also be used in integration tests to create new channels.

   Note that this should _not_ be used when relaying channel creation that
   are initiated by external users. For that purpose, use
   [`RelayChannelOpenHandshake`](crate::relay::impls::RelayChannelOpenHandshake),
   which would reuse the given channel ID instead of creating new ones.
*/

#[async_trait]
pub trait CanBootstrapChannel: HasRelayChains
where
    Self::SrcChain: HasInitChannelOptionsType<Self::DstChain>,
{
    async fn bootstrap_channel(
        &self,
        src_port_id: &SrcPortId<Self>,
        dst_port_id: &DstPortId<Self>,
        init_channel_options: &InitChannelOptions<Self::SrcChain, Self::DstChain>,
    ) -> Result<(SrcChannelId<Self>, DstChannelId<Self>), Self::Error>;
}

impl<Relay, SrcChain, DstChain> CanBootstrapChannel for Relay
where
    Relay: HasRelayChains<SrcChain = SrcChain, DstChain = DstChain>
        + CanInitChannel
        + CanRelayChannelOpenHandshake,
    SrcChain: HasInitChannelOptionsType<DstChain> + HasIbcChainTypes<DstChain> + HasAsyncErrorType,
    DstChain: HasIbcChainTypes<SrcChain> + HasAsyncErrorType,
{
    async fn bootstrap_channel(
        &self,
        src_port_id: &SrcPortId<Relay>,
        dst_port_id: &DstPortId<Relay>,
        init_channel_options: &SrcChain::InitChannelOptions,
    ) -> Result<(SrcChain::ChannelId, DstChain::ChannelId), Self::Error> {
        let src_channel_id = self
            .init_channel(src_port_id, dst_port_id, init_channel_options)
            .await?;

        let dst_channel_id = self
            .relay_channel_open_handshake(&src_channel_id, src_port_id, dst_port_id)
            .await?;

        Ok((src_channel_id, dst_channel_id))
    }
}
