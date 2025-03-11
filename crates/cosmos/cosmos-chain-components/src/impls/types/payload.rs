use cgp::prelude::*;
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
use hermes_relayer_components::chain::traits::types::create_client::{
    CreateClientPayloadTypeComponent, ProvideCreateClientPayloadType,
};
use hermes_relayer_components::chain::traits::types::packets::ack::AckPacketPayloadTypeProviderComponent;
use hermes_relayer_components::chain::traits::types::packets::receive::ReceivePacketPayloadTypeComponent;
use hermes_relayer_components::chain::traits::types::packets::timeout::TimeoutUnorderedPacketPayloadTypeComponent;
use hermes_relayer_components::chain::traits::types::update_client::{
    ProvideUpdateClientPayloadType, UpdateClientPayloadTypeComponent,
};

use crate::types::payloads::client::{CosmosCreateClientPayload, CosmosUpdateClientPayload};

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
            AckPacketPayloadTypeProviderComponent,
            TimeoutUnorderedPacketPayloadTypeComponent,
        ]:
            ProvidePacketPayloadTypes,
    }
}

#[cgp_provider(CreateClientPayloadTypeComponent)]
impl<Chain, Counterparty> ProvideCreateClientPayloadType<Chain, Counterparty>
    for ProvideCosmosPayloadTypes
where
    Chain: Async,
{
    type CreateClientPayload = CosmosCreateClientPayload;
}

#[cgp_provider(UpdateClientPayloadTypeComponent)]
impl<Chain, Counterparty> ProvideUpdateClientPayloadType<Chain, Counterparty>
    for ProvideCosmosPayloadTypes
where
    Chain: Async,
{
    type UpdateClientPayload = CosmosUpdateClientPayload;
}
