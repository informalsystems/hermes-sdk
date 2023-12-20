use alloc::sync::Arc;
use async_trait::async_trait;
use cgp_core::HasErrorType;
use cosmos_client_components::traits::message::{CosmosMessage, ToCosmosMessage};
use cosmos_client_components::types::channel::CosmosInitChannelOptions;
use cosmos_client_components::types::messages::channel::open_ack::CosmosChannelOpenAckMessage;
use cosmos_client_components::types::messages::channel::open_confirm::CosmosChannelOpenConfirmMessage;
use cosmos_client_components::types::messages::channel::open_init::CosmosChannelOpenInitMessage;
use cosmos_client_components::types::messages::channel::open_try::CosmosChannelOpenTryMessage;
use hermes_cosmos_relayer::types::error::{BaseError, Error};
use hermes_relayer_components::chain::traits::components::channel_handshake_message_builder::ChannelHandshakeMessageBuilder;
use hermes_relayer_components::chain::traits::types::channel::{
    HasChannelHandshakePayloads, HasInitChannelOptionsType,
};
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_relayer_types::core::ics04_channel::channel::{
    ChannelEnd, Counterparty as ChannelCounterparty, State,
};
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, PortId};

use crate::types::payloads::channel::{
    SolomachineChannelOpenAckPayload, SolomachineChannelOpenConfirmPayload,
    SolomachineChannelOpenTryPayload,
};

pub struct BuildSolomachineChannelHandshakeMessagesForCosmos;

#[async_trait]
impl<Chain, Counterparty> ChannelHandshakeMessageBuilder<Chain, Counterparty>
    for BuildSolomachineChannelHandshakeMessagesForCosmos
where
    Chain: HasInitChannelOptionsType<Counterparty, InitChannelOptions = CosmosInitChannelOptions>
        + HasIbcChainTypes<
            Counterparty,
            Message = Arc<dyn CosmosMessage>,
            ChannelId = ChannelId,
            PortId = PortId,
        > + HasErrorType<Error = Error>,
    Counterparty: HasChannelHandshakePayloads<
            Chain,
            ChannelOpenTryPayload = SolomachineChannelOpenTryPayload,
            ChannelOpenAckPayload = SolomachineChannelOpenAckPayload,
            ChannelOpenConfirmPayload = SolomachineChannelOpenConfirmPayload,
        > + HasIbcChainTypes<Chain, ChannelId = ChannelId, PortId = PortId>,
{
    async fn build_channel_open_init_message(
        _chain: &Chain,
        port_id: &PortId,
        counterparty_port_id: &PortId,
        init_channel_options: &CosmosInitChannelOptions,
    ) -> Result<Arc<dyn CosmosMessage>, Error> {
        let ordering = init_channel_options.ordering;
        let connection_hops = init_channel_options.connection_hops.clone();
        let channel_version = init_channel_options.channel_version.clone();
        let counterparty = ChannelCounterparty::new(counterparty_port_id.clone(), None);

        let channel = ChannelEnd::new(
            State::Init,
            ordering,
            counterparty,
            connection_hops,
            channel_version,
        );

        let message = CosmosChannelOpenInitMessage {
            port_id: port_id.clone(),
            channel,
        };

        Ok(message.to_cosmos_message())
    }

    async fn build_channel_open_try_message(
        _chain: &Chain,
        port_id: &PortId,
        counterparty_port_id: &PortId,
        counterparty_channel_id: &ChannelId,
        counterparty_payload: SolomachineChannelOpenTryPayload,
    ) -> Result<Arc<dyn CosmosMessage>, Error> {
        let proof_init = Vec::from(counterparty_payload.proof_init.serialize_compact())
            .try_into()
            .map_err(BaseError::proofs)?;

        let counterparty = ChannelCounterparty::new(
            counterparty_port_id.clone(),
            Some(counterparty_channel_id.clone()),
        );

        let channel = ChannelEnd::new(
            State::TryOpen,
            counterparty_payload.ordering,
            counterparty,
            counterparty_payload.connection_hops,
            counterparty_payload.version.clone(),
        );

        let message = CosmosChannelOpenTryMessage {
            port_id: port_id.clone(),
            channel,
            counterparty_version: counterparty_payload.version,
            update_height: counterparty_payload.update_height,
            proof_init,
        };

        Ok(message.to_cosmos_message())
    }

    async fn build_channel_open_ack_message(
        _chain: &Chain,
        port_id: &PortId,
        channel_id: &ChannelId,
        counterparty_channel_id: &ChannelId,
        counterparty_payload: SolomachineChannelOpenAckPayload,
    ) -> Result<Arc<dyn CosmosMessage>, Error> {
        let proof_try = Vec::from(counterparty_payload.proof_try.serialize_compact())
            .try_into()
            .map_err(BaseError::proofs)?;

        let message = CosmosChannelOpenAckMessage {
            port_id: port_id.clone(),
            channel_id: channel_id.clone(),
            counterparty_channel_id: counterparty_channel_id.clone(),
            counterparty_version: counterparty_payload.version,
            update_height: counterparty_payload.update_height,
            proof_try,
        };

        Ok(message.to_cosmos_message())
    }

    async fn build_channel_open_confirm_message(
        _chain: &Chain,
        port_id: &PortId,
        channel_id: &ChannelId,
        counterparty_payload: SolomachineChannelOpenConfirmPayload,
    ) -> Result<Arc<dyn CosmosMessage>, Error> {
        let proof_ack = Vec::from(counterparty_payload.proof_ack.serialize_compact())
            .try_into()
            .map_err(BaseError::proofs)?;

        let message = CosmosChannelOpenConfirmMessage {
            port_id: port_id.clone(),
            channel_id: channel_id.clone(),
            update_height: counterparty_payload.update_height,
            proof_ack,
        };

        Ok(message.to_cosmos_message())
    }
}
