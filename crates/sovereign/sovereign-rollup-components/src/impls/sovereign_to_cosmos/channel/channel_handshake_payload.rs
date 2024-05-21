use cgp_core::HasErrorType;
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, PortId};

use crate::types::height::RollupHeight;
use crate::types::payloads::channel::{
    SovereignChannelOpenAckRollupPayload, SovereignChannelOpenConfirmRollupPayload,
    SovereignChannelOpenTryRollupPayload,
};

pub struct BuildSovereignChannelHandshakePayload;

impl<Chain, Counterparty> ChannelHandshakePayloadBuilder<Chain, Counterparty>
    for BuildSovereignChannelHandshakePayload
where
    Chain: HasChannelHandshakePayloadTypes<
            Counterparty,
            ChannelOpenTryPayload = SovereignChannelOpenTryRollupPayload,
            ChannelOpenAckPayload = SovereignChannelOpenAckRollupPayload,
            ChannelOpenConfirmPayload = SovereignChannelOpenConfirmRollupPayload,
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
    ) -> Result<SovereignChannelOpenTryRollupPayload, Chain::Error> {
        todo!()
    }

    async fn build_channel_open_ack_payload(
        _chain: &Chain,
        _client_state: &Chain::ClientState,
        _height: &Chain::Height,
        _port_id: &Chain::PortId,
        _channel_id: &Chain::ChannelId,
    ) -> Result<SovereignChannelOpenAckRollupPayload, Chain::Error> {
        todo!()
    }

    async fn build_channel_open_confirm_payload(
        _chain: &Chain,
        _client_state: &Chain::ClientState,
        _height: &Chain::Height,
        _port_id: &Chain::PortId,
        _channel_id: &Chain::ChannelId,
    ) -> Result<SovereignChannelOpenConfirmRollupPayload, Chain::Error> {
        todo!()
    }
}
