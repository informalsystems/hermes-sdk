use cgp::prelude::*;

use crate::traits::commitment_prefix::HasCommitmentPrefixType;
use crate::traits::types::client_state::HasClientStateType;
use crate::traits::types::connection::{
    ConnectionOpenAckPayloadTypeComponent, ConnectionOpenConfirmPayloadTypeComponent,
    ConnectionOpenInitPayloadTypeComponent, ConnectionOpenTryPayloadTypeComponent,
    HasConnectionEndType, ProvideConnectionOpenAckPayloadType,
    ProvideConnectionOpenConfirmPayloadType, ProvideConnectionOpenInitPayloadType,
    ProvideConnectionOpenTryPayloadType,
};
use crate::traits::types::height::HasHeightType;
use crate::traits::types::proof::HasCommitmentProofType;
use crate::types::payloads::connection::{
    ConnectionOpenAckPayload, ConnectionOpenConfirmPayload, ConnectionOpenInitPayload,
    ConnectionOpenTryPayload,
};

pub struct ProvideConnectionPayloadTypes;

#[cgp_provider(ConnectionOpenInitPayloadTypeComponent)]
impl<Chain, Counterparty> ProvideConnectionOpenInitPayloadType<Chain, Counterparty>
    for ProvideConnectionPayloadTypes
where
    Chain: HasCommitmentPrefixType,
{
    type ConnectionOpenInitPayload = ConnectionOpenInitPayload<Chain>;
}

#[cgp_provider(ConnectionOpenTryPayloadTypeComponent)]
impl<Chain, Counterparty> ProvideConnectionOpenTryPayloadType<Chain, Counterparty>
    for ProvideConnectionPayloadTypes
where
    Chain: HasCommitmentPrefixType
        + HasCommitmentProofType
        + HasHeightType
        + HasConnectionEndType<Counterparty>,
    Counterparty: HasClientStateType<Chain> + HasHeightType,
{
    type ConnectionOpenTryPayload = ConnectionOpenTryPayload<Chain, Counterparty>;
}

#[cgp_provider(ConnectionOpenAckPayloadTypeComponent)]
impl<Chain, Counterparty> ProvideConnectionOpenAckPayloadType<Chain, Counterparty>
    for ProvideConnectionPayloadTypes
where
    Chain: HasCommitmentProofType + HasHeightType + HasConnectionEndType<Counterparty>,
    Counterparty: HasClientStateType<Chain> + HasHeightType,
{
    type ConnectionOpenAckPayload = ConnectionOpenAckPayload<Chain, Counterparty>;
}

#[cgp_provider(ConnectionOpenConfirmPayloadTypeComponent)]
impl<Chain, Counterparty> ProvideConnectionOpenConfirmPayloadType<Chain, Counterparty>
    for ProvideConnectionPayloadTypes
where
    Chain: HasCommitmentProofType + HasHeightType,
{
    type ConnectionOpenConfirmPayload = ConnectionOpenConfirmPayload<Chain>;
}
