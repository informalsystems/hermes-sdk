use hermes_core::chain_components::traits::{EvidenceTypeProvider, EvidenceTypeProviderComponent};
use hermes_prelude::*;
use ibc_client_tendermint::types::proto::v1::Misbehaviour;

pub struct ProvideTendermintEvidenceType;

#[cgp_provider(EvidenceTypeProviderComponent)]
impl<Chain> EvidenceTypeProvider<Chain> for ProvideTendermintEvidenceType
where
    Chain: Async,
{
    type Evidence = Misbehaviour;
}
