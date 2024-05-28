use cgp_core::{delegate_components, Async};
use hermes_relayer_components::chain::impls::types::payloads::channel::ProvideChannelPayloadTypes;
use hermes_relayer_components::chain::impls::types::payloads::connection::ProvideConnectionPayloadTypes;
use hermes_relayer_components::chain::impls::types::payloads::packet::ProvidePacketPayloadTypes;
use hermes_relayer_components::chain::traits::types::channel::{
    ChannelOpenAckPayloadTypeComponent, ChannelOpenConfirmPayloadTypeComponent,
    ChannelOpenTryPayloadTypeComponent,
};
use hermes_relayer_components::chain::traits::types::connection::{
    ConnectionOpenAckPayloadTypeComponent, ConnectionOpenConfirmPayloadTypeComponent,
    ConnectionOpenInitPayloadTypeComponent, ConnectionOpenTryPayloadTypeComponent,
};
use hermes_relayer_components::chain::traits::types::create_client::ProvideCreateClientPayloadType;
use hermes_relayer_components::chain::traits::types::packets::ack::ProvideAckPacketPayloadType;
use hermes_relayer_components::chain::traits::types::packets::receive::ReceivePacketPayloadTypeComponent;
use hermes_relayer_components::chain::traits::types::packets::timeout::ProvideTimeoutUnorderedPacketPayloadType;
use hermes_relayer_components::chain::traits::types::update_client::ProvideUpdateClientPayloadType;

use crate::types::payloads::client::{CosmosCreateClientPayload, CosmosUpdateClientPayload};
use crate::types::payloads::packet::{CosmosAckPacketPayload, CosmosTimeoutUnorderedPacketPayload};

pub struct ProvideCosmosPayloadTypes;

delegate_components! {
    ProvideCosmosPayloadTypes {
        [
            ConnectionOpenInitPayloadTypeComponent,
            ConnectionOpenTryPayloadTypeComponent,
            ConnectionOpenAckPayloadTypeComponent,
            ConnectionOpenConfirmPayloadTypeComponent,
        ]:
            ProvideConnectionPayloadTypes,
        [
            ChannelOpenTryPayloadTypeComponent,
            ChannelOpenAckPayloadTypeComponent,
            ChannelOpenConfirmPayloadTypeComponent,
        ]:
            ProvideChannelPayloadTypes,
        [
            ReceivePacketPayloadTypeComponent,
        ]:
            ProvidePacketPayloadTypes,
    }
}

impl<Chain, Counterparty> ProvideCreateClientPayloadType<Chain, Counterparty>
    for ProvideCosmosPayloadTypes
where
    Chain: Async,
{
    type CreateClientPayload = CosmosCreateClientPayload;
}

impl<Chain, Counterparty> ProvideUpdateClientPayloadType<Chain, Counterparty>
    for ProvideCosmosPayloadTypes
where
    Chain: Async,
{
    type UpdateClientPayload = CosmosUpdateClientPayload;
}

impl<Chain, Counterparty> ProvideAckPacketPayloadType<Chain, Counterparty>
    for ProvideCosmosPayloadTypes
where
    Chain: Async,
{
    type AckPacketPayload = CosmosAckPacketPayload;
}

impl<Chain, Counterparty> ProvideTimeoutUnorderedPacketPayloadType<Chain, Counterparty>
    for ProvideCosmosPayloadTypes
where
    Chain: Async,
{
    type TimeoutUnorderedPacketPayload = CosmosTimeoutUnorderedPacketPayload;
}
