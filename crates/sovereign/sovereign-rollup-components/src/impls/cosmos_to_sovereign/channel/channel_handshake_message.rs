use cgp_core::{CanRaiseError, HasErrorType};
use hermes_cosmos_chain_components::methods::encode::encode_to_any;
use hermes_relayer_components::chain::traits::message_builders::channel_handshake::{
    ChannelOpenAckMessageBuilder, ChannelOpenConfirmMessageBuilder, ChannelOpenInitMessageBuilder,
    ChannelOpenTryMessageBuilder,
};
use hermes_relayer_components::chain::traits::types::channel::{
    HasChannelEndType, HasChannelOpenAckPayloadType, HasChannelOpenConfirmPayloadType,
    HasChannelOpenTryPayloadType, HasInitChannelOptionsType,
};
use hermes_relayer_components::chain::traits::types::height::HasHeightFields;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::proof::HasCommitmentProofType;
use hermes_relayer_components::chain::types::channel_payload::{
    ChannelOpenAckPayload, ChannelOpenConfirmPayload, ChannelOpenTryPayload,
};
use ibc::core::channel::types::channel::{ChannelEnd, State};
use ibc_proto::ibc::core::channel::v1::{
    Channel, Counterparty as ChannelCounterparty, MsgChannelOpenAck, MsgChannelOpenConfirm,
    MsgChannelOpenInit, MsgChannelOpenTry,
};
use ibc_proto::ibc::core::client::v1::Height as ProtoHeight;
use ibc_relayer_types::core::ics02_client::error::Error as Ics02Error;
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, PortId};
use ibc_relayer_types::signer::Signer;

use crate::types::height::RollupHeight;
use crate::types::message::SovereignMessage;
use crate::types::messages::ibc::IbcMessageWithHeight;
use crate::types::payloads::channel::SovereignInitChannelOptions;

pub struct BuildCosmosChannelHandshakeMessageOnSovereign;

impl<Rollup, Counterparty> ChannelOpenInitMessageBuilder<Rollup, Counterparty>
    for BuildCosmosChannelHandshakeMessageOnSovereign
where
    Rollup: HasInitChannelOptionsType<Counterparty, InitChannelOptions = SovereignInitChannelOptions>
        + HasIbcChainTypes<
            Counterparty,
            Message = SovereignMessage,
            ChannelId = ChannelId,
            PortId = PortId,
        > + HasErrorType,
    Counterparty: HasIbcChainTypes<Rollup, ChannelId = ChannelId, PortId = PortId>,
{
    async fn build_channel_open_init_message(
        _rollup: &Rollup,
        port_id: &Rollup::PortId,
        counterparty_port_id: &Counterparty::PortId,
        init_channel_options: &SovereignInitChannelOptions,
    ) -> Result<SovereignMessage, Rollup::Error> {
        let port_id = port_id.clone();
        let ordering = init_channel_options.ordering;

        let connection_hops = init_channel_options
            .connection_hops
            .iter()
            .map(ToString::to_string)
            .collect();

        let channel_version = init_channel_options.channel_version.to_string();

        let channel = Channel {
            state: State::Init as i32,
            ordering: ordering as i32,
            counterparty: Some(ChannelCounterparty {
                port_id: counterparty_port_id.to_string(),
                channel_id: "".to_string(),
            }),
            connection_hops,
            version: channel_version,
            upgrade_sequence: 0,
        };

        let proto_message = MsgChannelOpenInit {
            port_id: port_id.to_string(),
            channel: channel.into(),
            signer: Signer::dummy().to_string(),
        };

        let any_message = encode_to_any("/ibc.core.channel.v1.MsgChannelOpenInit", &proto_message);

        Ok(IbcMessageWithHeight::new(any_message).into())
    }
}

impl<Rollup, Counterparty> ChannelOpenTryMessageBuilder<Rollup, Counterparty>
    for BuildCosmosChannelHandshakeMessageOnSovereign
where
    Rollup: HasIbcChainTypes<
            Counterparty,
            Message = SovereignMessage,
            ChannelId = ChannelId,
            PortId = PortId,
        > + CanRaiseError<Ics02Error>,
    Counterparty: HasChannelOpenTryPayloadType<
            Rollup,
            ChannelOpenTryPayload = ChannelOpenTryPayload<Counterparty, Rollup>,
        > + HasIbcChainTypes<Rollup, ChannelId = ChannelId, PortId = PortId>
        + HasChannelEndType<Rollup, ChannelEnd = ChannelEnd>
        + HasCommitmentProofType<CommitmentProof = Vec<u8>>
        + HasHeightFields,
{
    async fn build_channel_open_try_message(
        _rollup: &Rollup,
        port_id: &Rollup::PortId,
        counterparty_port_id: &Counterparty::PortId,
        counterparty_channel_id: &Counterparty::ChannelId,
        payload: ChannelOpenTryPayload<Counterparty, Rollup>,
    ) -> Result<SovereignMessage, Rollup::Error> {
        let ordering = payload.channel_end.ordering;
        let connection_hops = payload
            .channel_end
            .connection_hops
            .iter()
            .map(ToString::to_string)
            .collect();
        let version = payload.channel_end.version.to_string();

        let channel = Channel {
            state: State::TryOpen as i32,
            ordering: ordering as i32,
            counterparty: Some(ChannelCounterparty {
                port_id: counterparty_port_id.to_string(),
                channel_id: counterparty_channel_id.to_string(),
            }),
            connection_hops,
            version: version.clone(),
            upgrade_sequence: 0,
        };

        #[allow(deprecated)]
        let proto_message = MsgChannelOpenTry {
            port_id: port_id.to_string(),
            channel: channel.into(),
            counterparty_version: version.to_string(),
            proof_height: Some(ProtoHeight {
                revision_number: Counterparty::revision_number(&payload.update_height),
                revision_height: Counterparty::revision_height(&payload.update_height),
            }),
            proof_init: payload.proof_init,
            signer: Signer::dummy().to_string(),
            previous_channel_id: "".to_string(),
        };

        let any_message = encode_to_any("/ibc.core.channel.v1.MsgChannelOpenTry", &proto_message);

        Ok(IbcMessageWithHeight::new(any_message).into())
    }
}

impl<Rollup, Counterparty> ChannelOpenAckMessageBuilder<Rollup, Counterparty>
    for BuildCosmosChannelHandshakeMessageOnSovereign
where
    Rollup: HasIbcChainTypes<
            Counterparty,
            Message = SovereignMessage,
            ChannelId = ChannelId,
            PortId = PortId,
            Height = RollupHeight,
        > + CanRaiseError<Ics02Error>,
    Counterparty: HasChannelOpenAckPayloadType<
            Rollup,
            ChannelOpenAckPayload = ChannelOpenAckPayload<Counterparty, Rollup>,
        > + HasIbcChainTypes<Rollup, ChannelId = ChannelId, PortId = PortId>
        + HasChannelEndType<Rollup, ChannelEnd = ChannelEnd>
        + HasCommitmentProofType<CommitmentProof = Vec<u8>>
        + HasHeightFields,
{
    async fn build_channel_open_ack_message(
        _rollup: &Rollup,
        port_id: &Rollup::PortId,
        channel_id: &Rollup::ChannelId,
        counterparty_channel_id: &Counterparty::ChannelId,
        payload: ChannelOpenAckPayload<Counterparty, Rollup>,
    ) -> Result<SovereignMessage, Rollup::Error> {
        let proto_message = MsgChannelOpenAck {
            port_id: port_id.to_string(),
            channel_id: channel_id.to_string(),
            counterparty_channel_id: counterparty_channel_id.to_string(),
            counterparty_version: payload.channel_end.version.to_string(),
            proof_height: Some(ProtoHeight {
                revision_number: Counterparty::revision_number(&payload.update_height),
                revision_height: Counterparty::revision_height(&payload.update_height),
            }),
            proof_try: payload.proof_try,
            signer: Signer::dummy().to_string(),
        };

        let any_message = encode_to_any("/ibc.core.channel.v1.MsgChannelOpenAck", &proto_message);

        Ok(IbcMessageWithHeight::new(any_message).into())
    }
}

impl<Rollup, Counterparty> ChannelOpenConfirmMessageBuilder<Rollup, Counterparty>
    for BuildCosmosChannelHandshakeMessageOnSovereign
where
    Rollup: HasIbcChainTypes<
            Counterparty,
            Message = SovereignMessage,
            ChannelId = ChannelId,
            PortId = PortId,
            Height = RollupHeight,
        > + HasErrorType,
    Counterparty: HasChannelOpenConfirmPayloadType<
            Rollup,
            ChannelOpenConfirmPayload = ChannelOpenConfirmPayload<Counterparty>,
        > + HasIbcChainTypes<Rollup, ChannelId = ChannelId, PortId = PortId>
        + HasCommitmentProofType<CommitmentProof = Vec<u8>>
        + HasHeightFields,
{
    async fn build_channel_open_confirm_message(
        _rollup: &Rollup,
        port_id: &Rollup::PortId,
        channel_id: &Rollup::ChannelId,
        payload: ChannelOpenConfirmPayload<Counterparty>,
    ) -> Result<SovereignMessage, Rollup::Error> {
        let proto_message = MsgChannelOpenConfirm {
            port_id: port_id.to_string(),
            channel_id: channel_id.to_string(),
            proof_height: Some(ProtoHeight {
                revision_number: Counterparty::revision_number(&payload.update_height),
                revision_height: Counterparty::revision_height(&payload.update_height),
            }),
            proof_ack: payload.proof_ack,
            signer: Signer::dummy().to_string(),
        };

        let any_message = encode_to_any("/ibc.core.channel.v1.MsgChannelOpenAck", &proto_message);

        Ok(IbcMessageWithHeight::new(any_message).into())
    }
}
