use cgp_core::prelude::Async;
use hermes_relayer_components::chain::traits::types::channel::ProvideChannelHandshakePayloadTypes;
use hermes_relayer_components::chain::traits::types::connection::ProvideConnectionHandshakePayloadTypes;
use hermes_relayer_components::chain::traits::types::create_client::ProvideCreateClientPayloadType;
use hermes_relayer_components::chain::traits::types::packets::ack::ProvideAckPacketPayloadType;
use hermes_relayer_components::chain::traits::types::packets::receive::ProvideReceivePacketPayloadType;
use hermes_relayer_components::chain::traits::types::packets::timeout::ProvideTimeoutUnorderedPacketPayloadType;
use hermes_relayer_components::chain::traits::types::update_client::ProvideUpdateClientPayloadType;

use crate::types::payloads::channel::{
    SovereignChannelOpenAckPayload, SovereignChannelOpenConfirmPayload,
    SovereignChannelOpenTryPayload,
};
use crate::types::payloads::client::{SovereignCreateClientPayload, SovereignUpdateClientPayload};
use crate::types::payloads::connection::{
    SovereignConnectionOpenAckPayload, SovereignConnectionOpenConfirmPayload,
    SovereignConnectionOpenInitPayload, SovereignConnectionOpenTryPayload,
};
use crate::types::payloads::packet::{
    SovereignAckPacketPayload, SovereignReceivePacketPayload,
    SovereignTimeoutUnorderedPacketPayload,
};

pub struct ProvideSovereignPayloadTypes;

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

impl<Chain, Counterparty> ProvideChannelHandshakePayloadTypes<Chain, Counterparty>
    for ProvideSovereignPayloadTypes
where
    Chain: Async,
{
    type ChannelOpenTryPayload = SovereignChannelOpenTryPayload;

    type ChannelOpenAckPayload = SovereignChannelOpenAckPayload;

    type ChannelOpenConfirmPayload = SovereignChannelOpenConfirmPayload;
}

impl<Chain, Counterparty> ProvideConnectionHandshakePayloadTypes<Chain, Counterparty>
    for ProvideSovereignPayloadTypes
where
    Chain: Async,
{
    type ConnectionOpenInitPayload = SovereignConnectionOpenInitPayload;

    type ConnectionOpenTryPayload = SovereignConnectionOpenTryPayload;

    type ConnectionOpenAckPayload = SovereignConnectionOpenAckPayload;

    type ConnectionOpenConfirmPayload = SovereignConnectionOpenConfirmPayload;
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
