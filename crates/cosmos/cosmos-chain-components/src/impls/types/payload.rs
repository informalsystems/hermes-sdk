use cgp_core::{delegate_components, Async};
use hermes_relayer_components::chain::impls::types::connection_payload::ProvideConnectionPayloadTypes;
use hermes_relayer_components::chain::traits::types::channel::{
    ProvideChannelOpenAckPayloadType, ProvideChannelOpenConfirmPayloadType,
    ProvideChannelOpenTryPayloadType,
};
use hermes_relayer_components::chain::traits::types::connection::{
    ConnectionOpenAckPayloadTypeComponent, ConnectionOpenConfirmPayloadTypeComponent,
    ConnectionOpenInitPayloadTypeComponent, ConnectionOpenTryPayloadTypeComponent,
};
use hermes_relayer_components::chain::traits::types::create_client::ProvideCreateClientPayloadType;
use hermes_relayer_components::chain::traits::types::packets::ack::ProvideAckPacketPayloadType;
use hermes_relayer_components::chain::traits::types::packets::receive::ProvideReceivePacketPayloadType;
use hermes_relayer_components::chain::traits::types::packets::timeout::ProvideTimeoutUnorderedPacketPayloadType;
use hermes_relayer_components::chain::traits::types::update_client::ProvideUpdateClientPayloadType;

use crate::types::payloads::channel::{
    CosmosChannelOpenAckPayload, CosmosChannelOpenConfirmPayload, CosmosChannelOpenTryPayload,
};
use crate::types::payloads::client::{CosmosCreateClientPayload, CosmosUpdateClientPayload};
use crate::types::payloads::packet::{
    CosmosAckPacketPayload, CosmosReceivePacketPayload, CosmosTimeoutUnorderedPacketPayload,
};

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

impl<Chain, Counterparty> ProvideChannelOpenTryPayloadType<Chain, Counterparty>
    for ProvideCosmosPayloadTypes
where
    Chain: Async,
{
    type ChannelOpenTryPayload = CosmosChannelOpenTryPayload;
}

impl<Chain, Counterparty> ProvideChannelOpenAckPayloadType<Chain, Counterparty>
    for ProvideCosmosPayloadTypes
where
    Chain: Async,
{
    type ChannelOpenAckPayload = CosmosChannelOpenAckPayload;
}

impl<Chain, Counterparty> ProvideChannelOpenConfirmPayloadType<Chain, Counterparty>
    for ProvideCosmosPayloadTypes
where
    Chain: Async,
{
    type ChannelOpenConfirmPayload = CosmosChannelOpenConfirmPayload;
}

impl<Chain, Counterparty> ProvideReceivePacketPayloadType<Chain, Counterparty>
    for ProvideCosmosPayloadTypes
where
    Chain: Async,
{
    type ReceivePacketPayload = CosmosReceivePacketPayload;
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
