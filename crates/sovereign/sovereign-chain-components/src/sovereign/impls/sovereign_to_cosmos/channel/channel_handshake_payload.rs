use cgp_core::{CanRaiseError, HasErrorType};
use hermes_relayer_components::chain::traits::types::channel::HasChannelHandshakePayloadTypes;
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::{
    payload_builders::channel_handshake::ChannelHandshakePayloadBuilder,
    queries::chain_status::CanQueryChainHeight,
};
use hermes_sovereign_rollup_components::traits::json_rpc_client::HasJsonRpcClient;
use hermes_sovereign_rollup_components::types::height::RollupHeight;
use hermes_sovereign_rollup_components::types::rpc::height::HeightParam;
use ibc_query::core::channel::QueryChannelResponse;
use ibc_relayer_types::core::ics02_client::error::Error as ClientError;
use ibc_relayer_types::core::ics04_channel::version::Version;
use ibc_relayer_types::core::ics23_commitment::commitment::CommitmentProofBytes;
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, PortId};
use ibc_relayer_types::proofs::ProofError;
use ibc_relayer_types::Height;
use jsonrpsee::core::client::ClientT;
use serde::Serialize;

use crate::sovereign::{
    traits::chain::rollup::HasRollup,
    types::payloads::channel::{
        SovereignChannelOpenAckPayload, SovereignChannelOpenConfirmPayload,
        SovereignChannelOpenTryPayload,
    },
};

pub struct BuildSovereignChannelHandshakePayload;

impl<Chain, Counterparty, Rollup> ChannelHandshakePayloadBuilder<Chain, Counterparty>
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
        + HasRollup<Rollup = Rollup>
        + HasErrorType
        + CanRaiseError<ClientError>
        + CanRaiseError<ProofError>
        + CanRaiseError<Rollup::Error>,
    Rollup: CanQueryChainHeight<Height = RollupHeight> + HasJsonRpcClient,
    Rollup::JsonRpcClient: ClientT,
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
        chain: &Chain,
        _client_state: &Chain::ClientState,
        _height: &Chain::Height,
        port_id: &Chain::PortId,
        channel_id: &Chain::ChannelId,
    ) -> Result<SovereignChannelOpenAckPayload, Chain::Error> {
        let rollup_height = chain
            .rollup()
            .query_chain_height()
            .await
            .map_err(Chain::raise_error)?;

        let channel = query_channel(chain.rollup(), port_id, channel_id, &rollup_height).await;

        let version = Version::from(channel.channel.version.as_str().to_owned());

        let proof_try =
            CommitmentProofBytes::try_from(channel.proof).map_err(Chain::raise_error)?;

        let update_height = Height::new(
            channel.proof_height.revision_number(),
            channel.proof_height.revision_height(),
        )
        .map_err(Chain::raise_error)?;

        Ok(SovereignChannelOpenAckPayload {
            version,
            update_height,
            proof_try,
        })
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

pub async fn query_channel<'a, Rollup>(
    rollup: &Rollup,
    port_id: &'a PortId,
    channel_id: &'a ChannelId,
    rollup_height: &'a RollupHeight,
) -> QueryChannelResponse
where
    Rollup: HasJsonRpcClient,
    Rollup::JsonRpcClient: ClientT,
{
    #[derive(Serialize)]
    struct Request<'a> {
        pub port_id: &'a str,
        pub channel_id: &'a str,
        pub query_height: &'a HeightParam,
    }

    let request = Request {
        port_id: port_id.as_str(),
        channel_id: channel_id.as_str(),
        query_height: &rollup_height.into(),
    };

    rollup
        .json_rpc_client()
        .request("ibc_channel", (request,))
        .await
        .unwrap()
}
