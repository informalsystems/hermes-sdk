use hermes_relayer_components::chain::traits::payload_builders::channel_handshake::ChannelHandshakePayloadBuilder;
use hermes_relayer_components::chain::traits::types::channel::HasChannelHandshakePayloadTypes;
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer::chain::requests::{IncludeProof, QueryChannelRequest, QueryHeight};
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, PortId};
use ibc_relayer_types::Height;

use crate::traits::chain_handle::HasBlockingChainHandle;
use crate::types::payloads::channel::{
    CosmosChannelOpenAckPayload, CosmosChannelOpenConfirmPayload, CosmosChannelOpenTryPayload,
};

pub struct BuildCosmosChannelHandshakePayload;

impl<Chain, Counterparty> ChannelHandshakePayloadBuilder<Chain, Counterparty>
    for BuildCosmosChannelHandshakePayload
where
    Chain: HasChannelHandshakePayloadTypes<
            Counterparty,
            ChannelOpenTryPayload = CosmosChannelOpenTryPayload,
            ChannelOpenAckPayload = CosmosChannelOpenAckPayload,
            ChannelOpenConfirmPayload = CosmosChannelOpenConfirmPayload,
        > + HasIbcChainTypes<Counterparty, Height = Height, PortId = PortId, ChannelId = ChannelId>
        + HasClientStateType<Counterparty>
        + HasBlockingChainHandle,
{
    async fn build_channel_open_try_payload(
        chain: &Chain,
        _client_state: &Chain::ClientState,
        height: &Height,
        port_id: &PortId,
        channel_id: &ChannelId,
    ) -> Result<CosmosChannelOpenTryPayload, Chain::Error> {
        let height = *height;
        let port_id = port_id.clone();
        let channel_id = channel_id.clone();

        chain
            .with_blocking_chain_handle(move |chain_handle| {
                let (channel_end, _) = chain_handle
                    .query_channel(
                        QueryChannelRequest {
                            port_id: port_id.clone(),
                            channel_id: channel_id.clone(),
                            height: QueryHeight::Latest,
                        },
                        IncludeProof::No,
                    )
                    .map_err(Chain::raise_error)?;

                let proofs = chain_handle
                    .build_channel_proofs(&port_id, &channel_id, height)
                    .map_err(Chain::raise_error)?;

                let payload = CosmosChannelOpenTryPayload {
                    ordering: channel_end.ordering,
                    connection_hops: channel_end.connection_hops,
                    version: channel_end.version,
                    update_height: proofs.height(),
                    proof_init: proofs.object_proof().clone(),
                };

                Ok(payload)
            })
            .await
    }

    async fn build_channel_open_ack_payload(
        chain: &Chain,
        _client_state: &Chain::ClientState,
        height: &Height,
        port_id: &PortId,
        channel_id: &ChannelId,
    ) -> Result<CosmosChannelOpenAckPayload, Chain::Error> {
        let height = *height;
        let port_id = port_id.clone();
        let channel_id = channel_id.clone();

        chain
            .with_blocking_chain_handle(move |chain_handle| {
                let (channel_end, _) = chain_handle
                    .query_channel(
                        QueryChannelRequest {
                            port_id: port_id.clone(),
                            channel_id: channel_id.clone(),
                            height: QueryHeight::Latest,
                        },
                        IncludeProof::No,
                    )
                    .map_err(Chain::raise_error)?;

                let proofs = chain_handle
                    .build_channel_proofs(&port_id, &channel_id, height)
                    .map_err(Chain::raise_error)?;

                let payload = CosmosChannelOpenAckPayload {
                    version: channel_end.version,
                    update_height: proofs.height(),
                    proof_try: proofs.object_proof().clone(),
                };

                Ok(payload)
            })
            .await
    }

    async fn build_channel_open_confirm_payload(
        chain: &Chain,
        _client_state: &Chain::ClientState,
        height: &Height,
        port_id: &PortId,
        channel_id: &ChannelId,
    ) -> Result<CosmosChannelOpenConfirmPayload, Chain::Error> {
        let height = *height;
        let port_id = port_id.clone();
        let channel_id = channel_id.clone();

        chain
            .with_blocking_chain_handle(move |chain_handle| {
                let proofs = chain_handle
                    .build_channel_proofs(&port_id, &channel_id, height)
                    .map_err(Chain::raise_error)?;

                let payload = CosmosChannelOpenConfirmPayload {
                    update_height: proofs.height(),
                    proof_ack: proofs.object_proof().clone(),
                };

                Ok(payload)
            })
            .await
    }
}
