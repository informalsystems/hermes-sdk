use hermes_prelude::*;

#[cgp_type]
pub trait HasAckPacketPayloadType<Counterparty>: Async {
    type AckPacketPayload: Async;
}

#[cgp_type]
pub trait HasAcknowledgementType<Counterparty>: Async {
    type Acknowledgement: Async;
}

#[cgp_type]
pub trait HasAckCommitmentHashType {
    type AckCommitmentHash: Async;
}
