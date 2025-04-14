use cgp::prelude::*;
use hermes_relayer_components::chain::impls::{
    ProvideChannelPayloadTypes, ProvideConnectionPayloadTypes, ProvidePacketPayloadTypes,
};
use hermes_relayer_components::chain::traits::{
    AckPacketPayloadTypeProviderComponent, ChannelOpenAckPayloadTypeComponent,
    ChannelOpenConfirmPayloadTypeComponent, ChannelOpenTryPayloadTypeComponent,
    ConnectionOpenAckPayloadTypeComponent, ConnectionOpenConfirmPayloadTypeComponent,
    ConnectionOpenInitPayloadTypeComponent, ConnectionOpenTryPayloadTypeComponent,
    CreateClientPayloadTypeComponent, ProvideCreateClientPayloadType,
    ProvideUpdateClientPayloadType, ReceivePacketPayloadTypeComponent,
    TimeoutUnorderedPacketPayloadTypeComponent, UpdateClientPayloadTypeComponent,
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
