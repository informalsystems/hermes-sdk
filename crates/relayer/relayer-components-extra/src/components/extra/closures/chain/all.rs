use hermes_relayer_components::chain::traits::types::channel::HasChannelHandshakePayloadTypes;
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::consensus_state::HasConsensusStateType;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::packets::ack::HasAckPacketPayload;
use hermes_relayer_components::chain::traits::types::packets::receive::HasReceivePacketPayloadType;
use hermes_relayer_components::chain::traits::types::packets::timeout::HasTimeoutUnorderedPacketPayload;
use hermes_relayer_components::chain::traits::types::update_client::HasUpdateClientPayload;

use crate::components::extra::closures::chain::channel_handshake::UseExtraChainComponentsForChannelHandshake;
use crate::components::extra::closures::chain::event_relayer::UseExtraChainComponentsForEventRelayer;
use crate::components::extra::closures::chain::packet_relayer::UseExtraChainComponentsForPacketRelayer;

pub trait CanUseExtraChainComponents<Counterparty>:
    UseExtraChainComponentsForPacketRelayer<Counterparty>
    + UseExtraChainComponentsForEventRelayer<Counterparty>
    + UseExtraChainComponentsForChannelHandshake<Counterparty>
where
    Counterparty: HasHeightType
        + HasClientStateType<Self>
        + HasConsensusStateType<Self>
        + HasIbcChainTypes<Self>
        + HasUpdateClientPayload<Self>
        + HasReceivePacketPayloadType<Self>
        + HasAckPacketPayload<Self>
        + HasTimeoutUnorderedPacketPayload<Self>
        + HasChannelHandshakePayloadTypes<Self>,
{
}
