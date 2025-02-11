use cgp::prelude::*;

use crate::traits::types::channel::{
    ChannelOpenAckPayloadTypeComponent, ChannelOpenConfirmPayloadTypeComponent,
    ChannelOpenTryPayloadTypeComponent, HasChannelEndType, ProvideChannelOpenAckPayloadType,
    ProvideChannelOpenConfirmPayloadType, ProvideChannelOpenTryPayloadType,
};
use crate::traits::types::height::HasHeightType;
use crate::traits::types::proof::HasCommitmentProofType;
use crate::types::payloads::channel::{
    ChannelOpenAckPayload, ChannelOpenConfirmPayload, ChannelOpenTryPayload,
};

pub struct ProvideChannelPayloadTypes;

#[cgp_provider(ChannelOpenTryPayloadTypeComponent)]
impl<Chain, Counterparty> ProvideChannelOpenTryPayloadType<Chain, Counterparty>
    for ProvideChannelPayloadTypes
where
    Chain: HasChannelEndType<Counterparty> + HasHeightType + HasCommitmentProofType,
    Counterparty: Async,
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
