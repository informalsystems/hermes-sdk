use cgp::prelude::*;
use hermes_chain_type_components::traits::HasConnectionIdType;

use crate::traits::{
    ChannelOpenAckPayloadTypeComponent, ChannelOpenConfirmPayloadTypeComponent,
    ChannelOpenTryPayloadTypeComponent, HasChannelEndType, HasCommitmentProofType, HasHeightType,
    ProvideChannelOpenAckPayloadType, ProvideChannelOpenConfirmPayloadType,
    ProvideChannelOpenTryPayloadType,
};
use crate::types::payloads::channel::{
    ChannelOpenAckPayload, ChannelOpenConfirmPayload, ChannelOpenTryPayload,
};

pub struct ProvideChannelPayloadTypes;

#[cgp_provider(ChannelOpenTryPayloadTypeComponent)]
impl<Chain, Counterparty> ProvideChannelOpenTryPayloadType<Chain, Counterparty>
    for ProvideChannelPayloadTypes
where
    Chain: HasChannelEndType<Counterparty> + HasHeightType + HasCommitmentProofType,
    Counterparty: HasConnectionIdType<Chain> + Async,
{
    type ChannelOpenTryPayload = ChannelOpenTryPayload<Chain, Counterparty>;
}

#[cgp_provider(ChannelOpenAckPayloadTypeComponent)]
impl<Chain, Counterparty> ProvideChannelOpenAckPayloadType<Chain, Counterparty>
    for ProvideChannelPayloadTypes
where
    Chain: HasChannelEndType<Counterparty> + HasHeightType + HasCommitmentProofType,
    Counterparty: Async,
{
    type ChannelOpenAckPayload = ChannelOpenAckPayload<Chain, Counterparty>;
}

#[cgp_provider(ChannelOpenConfirmPayloadTypeComponent)]
impl<Chain, Counterparty> ProvideChannelOpenConfirmPayloadType<Chain, Counterparty>
    for ProvideChannelPayloadTypes
where
    Chain: HasHeightType + HasCommitmentProofType,
    Counterparty: Async,
{
    type ChannelOpenConfirmPayload = ChannelOpenConfirmPayload<Chain>;
}
