use cgp::core::Async;

use crate::chain::traits::types::channel::{
    HasChannelEndType, ProvideChannelOpenAckPayloadType, ProvideChannelOpenConfirmPayloadType,
    ProvideChannelOpenTryPayloadType,
};
use crate::chain::traits::types::height::HasHeightType;
use crate::chain::traits::types::proof::HasCommitmentProofType;
use crate::chain::types::payloads::channel::{
    ChannelOpenAckPayload, ChannelOpenConfirmPayload, ChannelOpenTryPayload,
};

pub struct ProvideChannelPayloadTypes;

impl<Chain, Counterparty> ProvideChannelOpenTryPayloadType<Chain, Counterparty>
    for ProvideChannelPayloadTypes
where
    Chain: HasChannelEndType<Counterparty> + HasHeightType + HasCommitmentProofType,
    Counterparty: Async,
{
    type ChannelOpenTryPayload = ChannelOpenTryPayload<Chain, Counterparty>;
}

impl<Chain, Counterparty> ProvideChannelOpenAckPayloadType<Chain, Counterparty>
    for ProvideChannelPayloadTypes
where
    Chain: HasChannelEndType<Counterparty> + HasHeightType + HasCommitmentProofType,
    Counterparty: Async,
{
    type ChannelOpenAckPayload = ChannelOpenAckPayload<Chain, Counterparty>;
}

impl<Chain, Counterparty> ProvideChannelOpenConfirmPayloadType<Chain, Counterparty>
    for ProvideChannelPayloadTypes
where
    Chain: HasHeightType + HasCommitmentProofType,
    Counterparty: Async,
{
    type ChannelOpenConfirmPayload = ChannelOpenConfirmPayload<Chain>;
}
