use cgp::prelude::*;
use hermes_chain_components::traits::HeightTypeProviderComponent;
use hermes_comet_light_client_components::traits::{
    VerdictTypeComponent, VerificationStatusTypeComponent,
};
use hermes_comet_light_client_components::types::{Verdict, VerificationStatus};
use tendermint::block::Height;

pub struct CometLightClientTypes;

delegate_components! {
    CometLightClientTypes {
        HeightTypeProviderComponent: Height,
        VerificationStatusTypeComponent: VerificationStatus,
        VerdictTypeComponent: Verdict,
    }
}
