use hermes_chain_components::traits::HeightTypeProviderComponent;
use hermes_comet_light_client_components::traits::{
    VerdictTypeComponent, VerificationStatusTypeComponent,
};
use hermes_comet_light_client_components::types::{Verdict, VerificationStatus};
use hermes_prelude::*;
use tendermint::block::Height;

pub struct CometLightClientTypes;

delegate_components! {
    CometLightClientTypes {
        HeightTypeProviderComponent: Height,
        VerificationStatusTypeComponent: VerificationStatus,
        VerdictTypeComponent: Verdict,
    }
}
