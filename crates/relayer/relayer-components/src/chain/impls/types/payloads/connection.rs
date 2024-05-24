use crate::chain::traits::commitment_prefix::HasCommitmentPrefixType;
use crate::chain::traits::types::client_state::HasClientStateType;
use crate::chain::traits::types::connection::{
    HasConnectionEndType, ProvideConnectionOpenAckPayloadType,
    ProvideConnectionOpenConfirmPayloadType, ProvideConnectionOpenInitPayloadType,
    ProvideConnectionOpenTryPayloadType,
};
use crate::chain::traits::types::height::HasHeightType;
use crate::chain::traits::types::proof::HasCommitmentProofType;
use crate::chain::types::payloads::connection::{
    ConnectionOpenAckPayload, ConnectionOpenConfirmPayload, ConnectionOpenInitPayload,
    ConnectionOpenTryPayload,
};

pub struct ProvideConnectionPayloadTypes;

impl<Chain, Counterparty> ProvideConnectionOpenInitPayloadType<Chain, Counterparty>
    for ProvideConnectionPayloadTypes
where
    Chain: HasCommitmentPrefixType,
{
    type ConnectionOpenInitPayload = ConnectionOpenInitPayload<Chain>;
}

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

impl<Chain, Counterparty> ProvideConnectionOpenAckPayloadType<Chain, Counterparty>
    for ProvideConnectionPayloadTypes
where
    Chain: HasCommitmentProofType + HasHeightType + HasConnectionEndType<Counterparty>,
    Counterparty: HasClientStateType<Chain> + HasHeightType,
{
    type ConnectionOpenAckPayload = ConnectionOpenAckPayload<Chain, Counterparty>;
}

impl<Chain, Counterparty> ProvideConnectionOpenConfirmPayloadType<Chain, Counterparty>
    for ProvideConnectionPayloadTypes
where
    Chain: HasCommitmentProofType + HasHeightType,
{
    type ConnectionOpenConfirmPayload = ConnectionOpenConfirmPayload<Chain>;
}
