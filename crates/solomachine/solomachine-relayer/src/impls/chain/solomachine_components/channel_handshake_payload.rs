use cgp_core::error::HasErrorType;
use eyre::eyre;
use hermes_error::types::Error;
use hermes_relayer_components::chain::traits::payload_builders::channel_handshake::{
    ChannelOpenAckPayloadBuilder, ChannelOpenConfirmPayloadBuilder, ChannelOpenTryPayloadBuilder,
};
use hermes_relayer_components::chain::traits::types::channel::{
    HasChannelOpenAckPayloadType, HasChannelOpenConfirmPayloadType, HasChannelOpenTryPayloadType,
};
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_relayer_types::core::ics04_channel::channel::State;
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, PortId};
use ibc_relayer_types::Height;

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
        + HasErrorType<Error = Error>,
{
    async fn build_channel_open_try_payload(
        chain: &Chain,
        client_state: &SolomachineClientState,
        height: &Height,
        port_id: &PortId,
        channel_id: &ChannelId,
    ) -> Result<SolomachineChannelOpenTryPayload, Error> {
        let channel = chain.query_channel(channel_id, port_id).await?;

        if channel.state != State::Init {
            return Err(Error::from(eyre!("expected channel to be in Init state")));
        }

        let ordering = *channel.ordering();
        let connection_hops = channel.connection_hops().clone();
        let version = channel.version().clone();

        let commitment_prefix: &str = chain.commitment_prefix();

        let channel_state_data =
            channel_proof_data(client_state, commitment_prefix, channel_id, channel)?;

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
        + HasErrorType<Error = Error>,
{
    async fn build_channel_open_ack_payload(
        chain: &Chain,
        client_state: &SolomachineClientState,
        height: &Height,
        port_id: &PortId,
        channel_id: &ChannelId,
    ) -> Result<SolomachineChannelOpenAckPayload, Error> {
        let channel = chain.query_channel(channel_id, port_id).await?;

        if channel.state != State::TryOpen {
            return Err(Error::from(eyre!(
                "expected channel to be in TryOpen state"
            )));
        }

        let version = channel.version().clone();

        let commitment_prefix = chain.commitment_prefix();

        let channel_state_data =
            channel_proof_data(client_state, commitment_prefix, channel_id, channel)?;

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
        + HasErrorType<Error = Error>,
{
    async fn build_channel_open_confirm_payload(
        chain: &Chain,
        client_state: &SolomachineClientState,
        height: &Height,
        port_id: &PortId,
        channel_id: &ChannelId,
    ) -> Result<SolomachineChannelOpenConfirmPayload, Chain::Error> {
        let channel = chain.query_channel(channel_id, port_id).await?;

        if !channel.state.is_open() {
            return Err(Error::from(eyre!("expected channel to be in open state")));
        }

        let commitment_prefix = chain.commitment_prefix();

        let channel_state_data =
            channel_proof_data(client_state, commitment_prefix, channel_id, channel)?;

        let secret_key = chain.secret_key();

        let channel_proof = sign_with_data(secret_key, &channel_state_data);

        let payload = SolomachineChannelOpenConfirmPayload {
            update_height: *height,
            proof_ack: channel_proof,
        };

        Ok(payload)
    }
}
