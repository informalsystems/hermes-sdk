use hermes_chain_components::traits::HeightTypeProviderComponent;
use hermes_comet_light_client_components::traits::{
    DivergenceTypeComponent, VerdictTypeComponent, VerificationStatusTypeComponent,
};
use hermes_comet_light_client_components::types::{Verdict, VerificationStatus};
use hermes_prelude::*;
use tendermint::block::Height;
use tendermint_light_client_detector::Divergence;

pub struct CometLightClientTypes;

delegate_components! {
    CometLightClientTypes {
        HeightTypeProviderComponent: Height,
        VerificationStatusTypeComponent: VerificationStatus,
        VerdictTypeComponent: Verdict,
        DivergenceTypeComponent: Divergence,
    }
}
