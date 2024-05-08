use cgp_core::HasErrorType;
use hermes_relayer_components::chain::traits::payload_builders::channel_handshake::ChannelHandshakePayloadBuilder;
use hermes_relayer_components::chain::traits::types::channel::HasChannelHandshakePayloadTypes;
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_sovereign_rollup_components::types::height::RollupHeight;
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, PortId};

use crate::sovereign::types::payloads::channel::{
    SovereignChannelOpenAckPayload, SovereignChannelOpenConfirmPayload,
    SovereignChannelOpenTryPayload,
};

pub struct BuildSovereignChannelHandshakePayload;

impl<Chain, Counterparty> ChannelHandshakePayloadBuilder<Chain, Counterparty>
    for BuildSovereignChannelHandshakePayload
where
    Chain: HasChannelHandshakePayloadTypes<
            Counterparty,
            ChannelOpenTryPayload = SovereignChannelOpenTryPayload,
            ChannelOpenAckPayload = SovereignChannelOpenAckPayload,
            ChannelOpenConfirmPayload = SovereignChannelOpenConfirmPayload,
        > + HasIbcChainTypes<
            Counterparty,
            Height = RollupHeight,
            ChannelId = ChannelId,
            PortId = PortId,
        > + HasClientStateType<Counterparty>
        + HasErrorType,
{
    async fn build_channel_open_try_payload(
        _chain: &Chain,
        _client_state: &Chain::ClientState,
        _height: &Chain::Height,
        _port_id: &Chain::PortId,
        _channel_id: &Chain::ChannelId,
    ) -> Result<SovereignChannelOpenTryPayload, Chain::Error> {
        todo!()
    }

    async fn build_channel_open_ack_payload(
        _chain: &Chain,
        _client_state: &Chain::ClientState,
        _height: &Chain::Height,
        _port_id: &Chain::PortId,
        _channel_id: &Chain::ChannelId,
    ) -> Result<SovereignChannelOpenAckPayload, Chain::Error> {
        todo!()
    }

    async fn build_channel_open_confirm_payload(
        _chain: &Chain,
        _client_state: &Chain::ClientState,
        _height: &Chain::Height,
        _port_id: &Chain::PortId,
        _channel_id: &Chain::ChannelId,
    ) -> Result<SovereignChannelOpenConfirmPayload, Chain::Error> {
        todo!()
    }
}
