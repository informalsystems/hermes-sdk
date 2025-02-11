use cgp::prelude::*;

use crate::traits::types::height::HasHeightType;
use crate::traits::types::packets::ack::{
    AckPacketPayloadTypeComponent, HasAcknowledgementType, ProvideAckPacketPayloadType,
};
use crate::traits::types::packets::receive::{
    ProvideReceivePacketPayloadType, ReceivePacketPayloadTypeComponent,
};
use crate::traits::types::packets::timeout::{
    ProvideTimeoutUnorderedPacketPayloadType, TimeoutUnorderedPacketPayloadTypeComponent,
};
use crate::traits::types::proof::HasCommitmentProofType;
use crate::types::payloads::packet::{
    AckPacketPayload, ReceivePacketPayload, TimeoutUnorderedPacketPayload,
};

pub struct ProvidePacketPayloadTypes;

#[cgp_provider(ReceivePacketPayloadTypeComponent)]
impl<Chain, Counterparty> ProvideReceivePacketPayloadType<Chain, Counterparty>
    for ProvidePacketPayloadTypes
where
    Chain: HasHeightType + HasCommitmentProofType,
{
    type ReceivePacketPayload = ReceivePacketPayload<Chain>;
}

#[cgp_provider(AckPacketPayloadTypeComponent)]
impl<Chain, Counterparty> ProvideAckPacketPayloadType<Chain, Counterparty>
    for ProvidePacketPayloadTypes
where
    Chain: HasHeightType + HasCommitmentProofType + HasAcknowledgementType<Counterparty>,
    Counterparty: Async,
{
    type AckPacketPayload = AckPacketPayload<Chain, Counterparty>;
}

#[cgp_provider(TimeoutUnorderedPacketPayloadTypeComponent)]
impl<Chain, Counterparty> ProvideTimeoutUnorderedPacketPayloadType<Chain, Counterparty>
    for ProvidePacketPayloadTypes
where
    Chain: HasHeightType + HasCommitmentProofType,
{
    type TimeoutUnorderedPacketPayload = TimeoutUnorderedPacketPayload<Chain>;
}
