use cgp::prelude::*;
use hermes_chain_type_components::traits::types::height::HasHeightType;
use hermes_chain_type_components::traits::types::ibc::channel_id::HasChannelIdType;
use hermes_chain_type_components::traits::types::ibc::connection_id::HasConnectionIdType;
use hermes_chain_type_components::traits::types::ibc::port_id::HasPortIdType;

use crate::traits::payload_builders::channel_handshake::{
    ChannelOpenAckPayloadBuilder, ChannelOpenAckPayloadBuilderComponent,
    ChannelOpenConfirmPayloadBuilder, ChannelOpenConfirmPayloadBuilderComponent,
    ChannelOpenTryPayloadBuilder, ChannelOpenTryPayloadBuilderComponent,
};
use crate::traits::queries::channel_end::CanQueryChannelEndWithProofs;
use crate::traits::queries::counterparty_connection_id::CanQueryCounterpartyConnectionId;
use crate::traits::types::channel::{
    HasChannelOpenAckPayloadType, HasChannelOpenConfirmPayloadType, HasChannelOpenTryPayloadType,
};
use crate::traits::types::client_state::HasClientStateType;
use crate::traits::types::proof::HasCommitmentProofHeight;
use crate::types::payloads::channel::{
    ChannelOpenAckPayload, ChannelOpenConfirmPayload, ChannelOpenTryPayload,
};

pub struct BuildChannelHandshakePayload;

#[cgp_provider(ChannelOpenTryPayloadBuilderComponent)]
impl<Chain, Counterparty> ChannelOpenTryPayloadBuilder<Chain, Counterparty>
    for BuildChannelHandshakePayload
where
    Chain: HasHeightType
        + HasChannelIdType<Counterparty>
        + HasPortIdType<Counterparty>
        + HasConnectionIdType<Counterparty>
        + HasChannelOpenTryPayloadType<
            Counterparty,
            ChannelOpenTryPayload = ChannelOpenTryPayload<Chain, Counterparty>,
        > + HasClientStateType<Counterparty>
        + CanQueryChannelEndWithProofs<Counterparty>
        + CanQueryCounterpartyConnectionId<Counterparty>
        + HasCommitmentProofHeight,
    Counterparty: HasConnectionIdType<Chain>,
{
    async fn build_channel_open_try_payload(
        chain: &Chain,
        _client_state: &Chain::ClientState,
        height: &Chain::Height,
        port_id: &Chain::PortId,
        channel_id: &Chain::ChannelId,
    ) -> Result<ChannelOpenTryPayload<Chain, Counterparty>, Chain::Error> {
        let (channel_end, proof_init) = chain
            .query_channel_end_with_proofs(channel_id, port_id, height)
            .await?;

        let counterparty_connection_id = chain
            .query_channel_end_counterparty_connection_id(&channel_end)
            .await?;

        // TODO: validate channel state

        // TODO: check that all commitment proof heights are the same
        let update_height = Chain::commitment_proof_height(&proof_init).clone();

        let payload = ChannelOpenTryPayload {
            channel_end,
            update_height,
            proof_init,
            counterparty_connection_id,
        };

        Ok(payload)
    }
}

#[cgp_provider(ChannelOpenAckPayloadBuilderComponent)]
impl<Chain, Counterparty> ChannelOpenAckPayloadBuilder<Chain, Counterparty>
    for BuildChannelHandshakePayload
where
    Chain: HasHeightType
        + HasChannelIdType<Counterparty>
        + HasPortIdType<Counterparty>
        + HasChannelOpenAckPayloadType<
            Counterparty,
            ChannelOpenAckPayload = ChannelOpenAckPayload<Chain, Counterparty>,
        > + HasClientStateType<Counterparty>
        + CanQueryChannelEndWithProofs<Counterparty>
        + HasCommitmentProofHeight,
{
    async fn build_channel_open_ack_payload(
        chain: &Chain,
        _client_state: &Chain::ClientState,
        height: &Chain::Height,
        port_id: &Chain::PortId,
        channel_id: &Chain::ChannelId,
    ) -> Result<ChannelOpenAckPayload<Chain, Counterparty>, Chain::Error> {
        let (channel_end, proof_try) = chain
            .query_channel_end_with_proofs(channel_id, port_id, height)
            .await?;

        // TODO: validate channel state

        // TODO: check that all commitment proof heights are the same
        let update_height = Chain::commitment_proof_height(&proof_try).clone();

        let payload = ChannelOpenAckPayload {
            channel_end,
            update_height,
            proof_try,
        };

        Ok(payload)
    }
}

#[cgp_provider(ChannelOpenConfirmPayloadBuilderComponent)]
impl<Chain, Counterparty> ChannelOpenConfirmPayloadBuilder<Chain, Counterparty>
    for BuildChannelHandshakePayload
where
    Chain: HasHeightType
        + HasChannelIdType<Counterparty>
        + HasPortIdType<Counterparty>
        + HasChannelOpenConfirmPayloadType<
            Counterparty,
            ChannelOpenConfirmPayload = ChannelOpenConfirmPayload<Chain>,
        > + HasClientStateType<Counterparty>
        + CanQueryChannelEndWithProofs<Counterparty>
        + HasCommitmentProofHeight,
{
    async fn build_channel_open_confirm_payload(
        chain: &Chain,
        _client_state: &Chain::ClientState,
        height: &Chain::Height,
        port_id: &Chain::PortId,
        channel_id: &Chain::ChannelId,
    ) -> Result<ChannelOpenConfirmPayload<Chain>, Chain::Error> {
        let (_channel_end, proof_ack) = chain
            .query_channel_end_with_proofs(channel_id, port_id, height)
            .await?;

        // TODO: validate channel state

        // TODO: check that all commitment proof heights are the same
        let update_height = Chain::commitment_proof_height(&proof_ack).clone();

        let payload = ChannelOpenConfirmPayload {
            update_height,
            proof_ack,
        };

        Ok(payload)
    }
}
