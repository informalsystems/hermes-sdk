use cgp::prelude::*;

use crate::traits::{
    AckPacketPayloadTypeProvider, AckPacketPayloadTypeProviderComponent, HasAcknowledgementType,
    HasCommitmentProofType, HasHeightType, ProvideReceivePacketPayloadType,
    ProvideTimeoutUnorderedPacketPayloadType, ReceivePacketPayloadTypeComponent,
    TimeoutUnorderedPacketPayloadTypeComponent,
};
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

#[cgp_provider(AckPacketPayloadTypeProviderComponent)]
impl<Chain, Counterparty> AckPacketPayloadTypeProvider<Chain, Counterparty>
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
