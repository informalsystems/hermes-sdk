use core::convert::Infallible;

use eyre::Report;
use hermes_chain_components::traits::HasHeightType;
use hermes_comet_light_client_components::impls::TargetLowerThanTrustedHeight;
use hermes_comet_light_client_components::traits::NoInitialTrustedState;
use hermes_error::handlers::{DebugError, HandleInfallible, ReportError, ReturnError};
use hermes_error::types::Error;
use hermes_prelude::*;
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
