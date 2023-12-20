use ibc_relayer_components::chain::traits::types::channel::HasChannelHandshakePayloads;
use ibc_relayer_components::chain::traits::types::client_state::HasClientStateType;
use ibc_relayer_components::chain::traits::types::consensus_state::HasConsensusStateType;
use ibc_relayer_components::chain::traits::types::height::HasHeightType;
use ibc_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_relayer_components::chain::traits::types::packets::ack::HasAckPacketPayload;
use ibc_relayer_components::chain::traits::types::packets::receive::HasReceivePacketPayload;
use ibc_relayer_components::chain::traits::types::packets::timeout::HasTimeoutUnorderedPacketPayload;
use ibc_relayer_components::chain::traits::types::update_client::HasUpdateClientPayload;

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
        + HasReceivePacketPayload<Self>
        + HasAckPacketPayload<Self>
        + HasTimeoutUnorderedPacketPayload<Self>
        + HasChannelHandshakePayloads<Self>,
{
}
