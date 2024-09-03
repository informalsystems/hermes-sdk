use cgp::core::Async;

use crate::chain::traits::types::height::HasHeightType;
use crate::chain::traits::types::packets::ack::{
    HasAcknowledgementType, ProvideAckPacketPayloadType,
};
use crate::chain::traits::types::packets::receive::ProvideReceivePacketPayloadType;
use crate::chain::traits::types::packets::timeout::ProvideTimeoutUnorderedPacketPayloadType;
use crate::chain::traits::types::proof::HasCommitmentProofType;
use crate::chain::types::payloads::packet::{
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
