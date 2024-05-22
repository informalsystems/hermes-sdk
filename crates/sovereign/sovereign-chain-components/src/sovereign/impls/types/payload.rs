use cgp_core::prelude::Async;
use hermes_relayer_components::chain::traits::types::channel::ProvideInitChannelOptionsType;
use hermes_relayer_components::chain::traits::types::connection::ProvideInitConnectionOptionsType;
use hermes_relayer_components::chain::traits::types::create_client::{
    ProvideCreateClientOptionsType, ProvideCreateClientPayloadType,
};
use hermes_relayer_components::chain::traits::types::packets::ack::ProvideAckPacketPayloadType;
use hermes_relayer_components::chain::traits::types::packets::receive::ProvideReceivePacketPayloadType;
use hermes_relayer_components::chain::traits::types::packets::timeout::ProvideTimeoutUnorderedPacketPayloadType;
use hermes_relayer_components::chain::traits::types::update_client::ProvideUpdateClientPayloadType;
use hermes_sovereign_rollup_components::types::payloads::channel::SovereignInitChannelOptions;
use hermes_sovereign_rollup_components::types::payloads::connection::SovereignInitConnectionOptions;

use crate::sovereign::types::payloads::client::{
    SovereignCreateClientOptions, SovereignCreateClientPayload, SovereignUpdateClientPayload,
};
use crate::sovereign::types::payloads::packet::{
    SovereignAckPacketPayload, SovereignReceivePacketPayload,
    SovereignTimeoutUnorderedPacketPayload,
};

pub struct ProvideSovereignPayloadTypes;

impl<Chain, Counterparty> ProvideCreateClientOptionsType<Chain, Counterparty>
    for ProvideSovereignPayloadTypes
where
    Chain: Async,
{
    type CreateClientOptions = SovereignCreateClientOptions;
}

impl<Chain, Counterparty> ProvideCreateClientPayloadType<Chain, Counterparty>
    for ProvideSovereignPayloadTypes
where
    Chain: Async,
{
    type CreateClientPayload = SovereignCreateClientPayload;
}

impl<Chain, Counterparty> ProvideUpdateClientPayloadType<Chain, Counterparty>
    for ProvideSovereignPayloadTypes
where
    Chain: Async,
{
    type UpdateClientPayload = SovereignUpdateClientPayload;
}

impl<Chain, Counterparty> ProvideInitConnectionOptionsType<Chain, Counterparty>
    for ProvideSovereignPayloadTypes
where
    Chain: Async,
{
    type InitConnectionOptions = SovereignInitConnectionOptions;
}

impl<Chain, Counterparty> ProvideInitChannelOptionsType<Chain, Counterparty>
    for ProvideSovereignPayloadTypes
where
    Chain: Async,
{
    type InitChannelOptions = SovereignInitChannelOptions;
}

impl<Chain, Counterparty> ProvideReceivePacketPayloadType<Chain, Counterparty>
    for ProvideSovereignPayloadTypes
where
    Chain: Async,
{
    type ReceivePacketPayload = SovereignReceivePacketPayload;
}

impl<Chain, Counterparty> ProvideAckPacketPayloadType<Chain, Counterparty>
    for ProvideSovereignPayloadTypes
where
    Chain: Async,
{
    type AckPacketPayload = SovereignAckPacketPayload;
}

impl<Chain, Counterparty> ProvideTimeoutUnorderedPacketPayloadType<Chain, Counterparty>
    for ProvideSovereignPayloadTypes
where
    Chain: Async,
{
    type TimeoutUnorderedPacketPayload = SovereignTimeoutUnorderedPacketPayload;
}
