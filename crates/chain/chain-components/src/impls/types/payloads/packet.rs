use cgp::core::Async;

use crate::traits::types::height::HasHeightType;
use crate::traits::types::packets::ack::{HasAcknowledgementType, ProvideAckPacketPayloadType};
use crate::traits::types::packets::receive::ProvideReceivePacketPayloadType;
use crate::traits::types::packets::timeout::ProvideTimeoutUnorderedPacketPayloadType;
use crate::traits::types::proof::HasCommitmentProofType;
use crate::types::payloads::packet::{
    AckPacketPayload, ReceivePacketPayload, TimeoutUnorderedPacketPayload,
};

pub struct ProvidePacketPayloadTypes;

impl<Chain, Counterparty> ProvideReceivePacketPayloadType<Chain, Counterparty>
    for ProvidePacketPayloadTypes
where
    Chain: HasHeightType + HasCommitmentProofType,
{
    type ReceivePacketPayload = ReceivePacketPayload<Chain>;
}

impl<Chain, Counterparty> ProvideAckPacketPayloadType<Chain, Counterparty>
    for ProvidePacketPayloadTypes
where
    Chain: HasHeightType + HasCommitmentProofType + HasAcknowledgementType<Counterparty>,
    Counterparty: Async,
{
    type AckPacketPayload = AckPacketPayload<Chain, Counterparty>;
}

impl<Chain, Counterparty> ProvideTimeoutUnorderedPacketPayloadType<Chain, Counterparty>
    for ProvidePacketPayloadTypes
where
    Chain: HasHeightType + HasCommitmentProofType,
{
    type TimeoutUnorderedPacketPayload = TimeoutUnorderedPacketPayload<Chain>;
}
