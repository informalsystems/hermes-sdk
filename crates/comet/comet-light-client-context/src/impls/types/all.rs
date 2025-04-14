use cgp::prelude::*;
use hermes_chain_components::traits::HeightTypeProviderComponent;
use hermes_comet_light_client_components::traits::types::status::VerificationStatusTypeComponent;
use hermes_comet_light_client_components::traits::types::verdict::VerdictTypeComponent;
use hermes_comet_light_client_components::types::status::VerificationStatus;
use hermes_comet_light_client_components::types::verdict::Verdict;
use tendermint::block::Height;

pub struct CometLightClientTypes;

delegate_components! {
    CometLightClientTypes {
        HeightTypeProviderComponent: Height,
        VerificationStatusTypeComponent: VerificationStatus,
        VerdictTypeComponent: Verdict,
    }
}
