use core::convert::Infallible;

use cgp::prelude::*;
use eyre::Report;
use hermes_chain_components::traits::types::height::HasHeightType;
use hermes_comet_light_client_components::impls::verify_target_height::verify_forward::TargetLowerThanTrustedHeight;
use hermes_comet_light_client_components::traits::verify_target_height::NoInitialTrustedState;
use hermes_error::handlers::debug::DebugError;
use hermes_error::handlers::identity::ReturnError;
use hermes_error::handlers::infallible::HandleInfallible;
use hermes_error::handlers::report::ReportError;
use hermes_error::types::Error;
use tendermint_light_client_verifier::errors::VerificationErrorDetail;

use crate::impls::validate_light_block::TrustedStateOutsideTrustingPeriod;

pub struct HandleLightClientError;

delegate_components! {
    HandleLightClientError {
        Error: ReturnError,
        Infallible: HandleInfallible,
        [
            Report,
            tendermint::Error,
            tendermint_rpc::Error,
        ]: ReportError,
        [
            NoInitialTrustedState,
            VerificationErrorDetail,
            <'a> TrustedStateOutsideTrustingPeriod<'a>,
            <'a, Client: HasHeightType> TargetLowerThanTrustedHeight<'a, Client>,
        ]:
            DebugError,
    }
}
