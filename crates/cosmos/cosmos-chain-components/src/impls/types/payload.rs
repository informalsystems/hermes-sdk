use hermes_core::chain_components::traits::{
    ProvideRecoverClientPayloadType, ProvideUpgradeClientPayloadType,
    RecoverClientPayloadTypeComponent, UpgradeClientPayloadTypeComponent,
};
use hermes_core::relayer_components::chain::impls::{
    ProvideChannelPayloadTypes, ProvideConnectionPayloadTypes, ProvidePacketPayloadTypes,
};
use hermes_core::relayer_components::chain::traits::{
    AckPacketPayloadTypeProviderComponent, ChannelOpenAckPayloadTypeComponent,
    ChannelOpenConfirmPayloadTypeComponent, ChannelOpenTryPayloadTypeComponent,
    ConnectionOpenAckPayloadTypeComponent, ConnectionOpenConfirmPayloadTypeComponent,
    ConnectionOpenInitPayloadTypeComponent, ConnectionOpenTryPayloadTypeComponent,
    CreateClientPayloadTypeComponent, ProvideCreateClientPayloadType,
    ProvideUpdateClientPayloadType, ReceivePacketPayloadTypeComponent,
    TimeoutUnorderedPacketPayloadTypeComponent, UpdateClientPayloadTypeComponent,
};
use hermes_prelude::*;

use crate::impls::{CosmosRecoverClientPayload, CosmosUpgradeClientPayload};
use crate::types::{CosmosCreateClientPayload, CosmosUpdateClientPayload};

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

#[cgp_provider(RecoverClientPayloadTypeComponent)]
impl<Chain> ProvideRecoverClientPayloadType<Chain> for ProvideCosmosPayloadTypes
where
    Chain: Async,
{
    type RecoverClientPayload = CosmosRecoverClientPayload;
}

#[cgp_provider(UpgradeClientPayloadTypeComponent)]
impl<Chain> ProvideUpgradeClientPayloadType<Chain> for ProvideCosmosPayloadTypes
where
    Chain: Async,
{
    type UpgradeClientPayload = CosmosUpgradeClientPayload;
}
