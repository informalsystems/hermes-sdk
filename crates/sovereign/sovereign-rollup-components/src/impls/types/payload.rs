use cgp_core::prelude::Async;
use hermes_relayer_components::chain::traits::types::channel::{
    ProvideChannelOpenAckPayloadType, ProvideChannelOpenConfirmPayloadType,
    ProvideChannelOpenTryPayloadType, ProvideInitChannelOptionsType,
};
use hermes_relayer_components::chain::traits::types::connection::ProvideInitConnectionOptionsType;
use hermes_relayer_components::chain::traits::types::create_client::{
    ProvideCreateClientOptionsType, ProvideCreateClientPayloadType,
};
use hermes_relayer_components::chain::traits::types::packets::ack::ProvideAckPacketPayloadType;
use hermes_relayer_components::chain::traits::types::packets::receive::ProvideReceivePacketPayloadType;
use hermes_relayer_components::chain::traits::types::packets::timeout::ProvideTimeoutUnorderedPacketPayloadType;
use hermes_relayer_components::chain::traits::types::update_client::ProvideUpdateClientPayloadType;
use ibc_relayer::chain::client::ClientSettings;

use crate::types::payloads::channel::SovereignInitChannelOptions;
use crate::types::payloads::client::{SovereignCreateClientPayload, SovereignUpdateClientPayload};
use crate::types::payloads::connection::SovereignInitConnectionOptions;
use crate::types::payloads::packet::{
    SovereignAckPacketRollupPayload, SovereignReceivePacketRollupPayload,
    SovereignTimeoutUnorderedPacketRollupPayload,
};

pub struct ProvideSovereignRollupPayloadTypes;

impl<Chain, Counterparty> ProvideCreateClientOptionsType<Chain, Counterparty>
    for ProvideSovereignRollupPayloadTypes
where
    Chain: Async,
{
    type CreateClientOptions = ClientSettings;
}

impl<Chain, Counterparty> ProvideCreateClientPayloadType<Chain, Counterparty>
    for ProvideSovereignRollupPayloadTypes
where
    Chain: Async,
{
    type CreateClientPayload = SovereignCreateClientPayload;
}

impl<Chain, Counterparty> ProvideUpdateClientPayloadType<Chain, Counterparty>
    for ProvideSovereignRollupPayloadTypes
where
    Chain: Async,
{
    type UpdateClientPayload = SovereignUpdateClientPayload;
}

impl<Chain, Counterparty> ProvideInitConnectionOptionsType<Chain, Counterparty>
    for ProvideSovereignRollupPayloadTypes
where
    Chain: Async,
{
    type InitConnectionOptions = SovereignInitConnectionOptions;
}

impl<Chain, Counterparty> ProvideInitChannelOptionsType<Chain, Counterparty>
    for ProvideSovereignRollupPayloadTypes
where
    Chain: Async,
{
    type InitChannelOptions = SovereignInitChannelOptions;
}

impl<Chain, Counterparty> ProvideReceivePacketPayloadType<Chain, Counterparty>
    for ProvideSovereignRollupPayloadTypes
where
    Chain: Async,
{
    type ReceivePacketPayload = SovereignReceivePacketRollupPayload;
}

impl<Chain, Counterparty> ProvideAckPacketPayloadType<Chain, Counterparty>
    for ProvideSovereignRollupPayloadTypes
where
    Chain: Async,
{
    type AckPacketPayload = SovereignAckPacketRollupPayload;
}

impl<Chain, Counterparty> ProvideTimeoutUnorderedPacketPayloadType<Chain, Counterparty>
    for ProvideSovereignRollupPayloadTypes
where
    Chain: Async,
{
    type TimeoutUnorderedPacketPayload = SovereignTimeoutUnorderedPacketRollupPayload;
}
