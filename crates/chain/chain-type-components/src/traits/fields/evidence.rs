use hermes_prelude::*;

use crate::traits::{HasClientIdType, HasEvidenceType};

#[cgp_component {
    provider: EvidenceFieldsGetter,
    context: Chain,
}]
pub trait HasEvidenceFields<Counterparty>: HasEvidenceType + HasClientIdType<Counterparty> {
    fn evidence_client_id(evidence: &Self::Evidence) -> Self::ClientId;
}
