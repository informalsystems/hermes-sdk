use cgp::core::error::CanRaiseError;
use hermes_relayer_components::chain::traits::commitment_prefix::HasIbcCommitmentPrefix;
use hermes_relayer_components::chain::traits::payload_builders::channel_handshake::{
    ChannelOpenAckPayloadBuilder, ChannelOpenConfirmPayloadBuilder, ChannelOpenTryPayloadBuilder,
};
use hermes_relayer_components::chain::traits::queries::channel_end::CanQueryChannelEnd;
use hermes_relayer_components::chain::traits::types::channel::{
    HasChannelOpenAckPayloadType, HasChannelOpenConfirmPayloadType, HasChannelOpenTryPayloadType,
};
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc::core::channel::types::channel::{ChannelEnd, State};
use ibc::core::client::types::Height;
use ibc::core::host::types::identifiers::{ChannelId, PortId};

use crate::methods::encode::sign_data::sign_with_data;
use crate::methods::proofs::channel::channel_proof_data;
use crate::traits::solomachine::Solomachine;
use crate::types::client_state::SolomachineClientState;
use crate::types::payloads::channel::{
    SolomachineChannelOpenAckPayload, SolomachineChannelOpenConfirmPayload,
    SolomachineChannelOpenTryPayload,
};

pub struct BuildSolomachineChannelHandshakePayloads;

impl<Chain, Counterparty> ChannelOpenTryPayloadBuilder<Chain, Counterparty>
    for BuildSolomachineChannelHandshakePayloads
where
    Chain: Solomachine
        + HasIbcChainTypes<Counterparty, Height = Height, PortId = PortId, ChannelId = ChannelId>
        + HasChannelOpenTryPayloadType<
            Counterparty,
            ChannelOpenTryPayload = SolomachineChannelOpenTryPayload,
        > + HasClientStateType<Counterparty, ClientState = SolomachineClientState>
        + CanQueryChannelEnd<Counterparty, ChannelEnd = ChannelEnd>
        + HasIbcCommitmentPrefix<CommitmentPrefix = String>
        + CanRaiseError<&'static str>,
{
    async fn build_channel_open_try_payload(
        chain: &Chain,
        client_state: &SolomachineClientState,
        height: &Height,
        port_id: &PortId,
        channel_id: &ChannelId,
    ) -> Result<SolomachineChannelOpenTryPayload, Chain::Error> {
        let channel = chain.query_channel_end(channel_id, port_id, height).await?;

        if channel.state != State::Init {
            return Err(Chain::raise_error("expected channel to be in Init state"));
        }

        let ordering = *channel.ordering();
        let connection_hops = channel.connection_hops().clone();
        let version = channel.version().clone();

        let commitment_prefix: &str = chain.ibc_commitment_prefix();

        let channel_state_data =
            channel_proof_data(client_state, commitment_prefix, channel_id, channel);

        let secret_key = chain.secret_key();

        let channel_proof = sign_with_data(secret_key, &channel_state_data);

        let payload = SolomachineChannelOpenTryPayload {
            ordering,
            connection_hops,
            version,
            update_height: *height,
            proof_init: channel_proof,
        };

        Ok(payload)
    }
}

impl<Chain, Counterparty> ChannelOpenAckPayloadBuilder<Chain, Counterparty>
    for BuildSolomachineChannelHandshakePayloads
where
    Chain: Solomachine
        + HasIbcChainTypes<Counterparty, Height = Height, PortId = PortId, ChannelId = ChannelId>
        + HasChannelOpenAckPayloadType<
            Counterparty,
            ChannelOpenAckPayload = SolomachineChannelOpenAckPayload,
        > + HasClientStateType<Counterparty, ClientState = SolomachineClientState>
        + CanQueryChannelEnd<Counterparty, ChannelEnd = ChannelEnd>
        + HasIbcCommitmentPrefix<CommitmentPrefix = String>
        + CanRaiseError<&'static str>,
{
    async fn build_channel_open_ack_payload(
        chain: &Chain,
        client_state: &SolomachineClientState,
        height: &Height,
        port_id: &PortId,
        channel_id: &ChannelId,
    ) -> Result<SolomachineChannelOpenAckPayload, Chain::Error> {
        let channel = chain.query_channel_end(channel_id, port_id, height).await?;

        if channel.state != State::TryOpen {
            return Err(Chain::raise_error(
                "expected channel to be in TryOpen state",
            ));
        }

        let version = channel.version().clone();

        let commitment_prefix = chain.ibc_commitment_prefix();

        let channel_state_data =
            channel_proof_data(client_state, commitment_prefix, channel_id, channel);

        let secret_key = chain.secret_key();

        let channel_proof = sign_with_data(secret_key, &channel_state_data);

        let payload = SolomachineChannelOpenAckPayload {
            version,
            update_height: *height,
            proof_try: channel_proof,
        };

        Ok(payload)
    }
}

impl<Chain, Counterparty> ChannelOpenConfirmPayloadBuilder<Chain, Counterparty>
    for BuildSolomachineChannelHandshakePayloads
where
    Chain: Solomachine
        + HasIbcChainTypes<Counterparty, Height = Height, PortId = PortId, ChannelId = ChannelId>
        + HasChannelOpenConfirmPayloadType<
            Counterparty,
            ChannelOpenConfirmPayload = SolomachineChannelOpenConfirmPayload,
        > + HasClientStateType<Counterparty, ClientState = SolomachineClientState>
        + CanQueryChannelEnd<Counterparty, ChannelEnd = ChannelEnd>
        + HasIbcCommitmentPrefix<CommitmentPrefix = String>
        + CanRaiseError<&'static str>,
{
    async fn build_channel_open_confirm_payload(
        chain: &Chain,
        client_state: &SolomachineClientState,
        height: &Height,
        port_id: &PortId,
        channel_id: &ChannelId,
    ) -> Result<SolomachineChannelOpenConfirmPayload, Chain::Error> {
        let channel = chain.query_channel_end(channel_id, port_id, height).await?;

        if !channel.state.is_open() {
            return Err(Chain::raise_error("expected channel to be in open state"));
        }

        let commitment_prefix = chain.ibc_commitment_prefix();

        let channel_state_data =
            channel_proof_data(client_state, commitment_prefix, channel_id, channel);

        let secret_key = chain.secret_key();

        let channel_proof = sign_with_data(secret_key, &channel_state_data);

        let payload = SolomachineChannelOpenConfirmPayload {
            update_height: *height,
            proof_ack: channel_proof,
        };

        Ok(payload)
    }
}
