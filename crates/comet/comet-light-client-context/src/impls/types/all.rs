use hermes_chain_components::traits::{EvidenceTypeProviderComponent, HeightTypeProviderComponent};
use hermes_comet_light_client_components::traits::{
    DivergenceTypeProviderComponent, VerdictTypeComponent, VerificationStatusTypeComponent,
};
use hermes_comet_light_client_components::types::{Verdict, VerificationStatus};
use hermes_prelude::*;
use ibc_client_tendermint::types::proto::v1::Misbehaviour;
use tendermint::block::Height;
use tendermint_light_client_detector::Divergence;

pub struct CometLightClientTypes;

delegate_components! {
    CometLightClientTypes {
        HeightTypeProviderComponent: Height,
        VerificationStatusTypeComponent: VerificationStatus,
        VerdictTypeComponent: Verdict,
        DivergenceTypeProviderComponent: Divergence,
        EvidenceTypeProviderComponent: Misbehaviour,
    }
}
