use cgp::core::Async;
use hermes_chain_type_components::traits::types::ibc::connection_id::HasConnectionIdType;

use crate::traits::types::channel::{
    HasChannelEndType, ProvideChannelOpenAckPayloadType, ProvideChannelOpenConfirmPayloadType,
    ProvideChannelOpenTryPayloadType,
};
use crate::traits::types::height::HasHeightType;
use crate::traits::types::proof::HasCommitmentProofType;
use crate::types::payloads::channel::{
    ChannelOpenAckPayload, ChannelOpenConfirmPayload, ChannelOpenTryPayload,
};

pub struct ProvideChannelPayloadTypes;

impl<Chain, Counterparty> ProvideChannelOpenTryPayloadType<Chain, Counterparty>
    for ProvideChannelPayloadTypes
where
    Chain: HasChannelEndType<Counterparty> + HasHeightType + HasCommitmentProofType,
    Counterparty: HasConnectionIdType<Chain> + Async,
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
