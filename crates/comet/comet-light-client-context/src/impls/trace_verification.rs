use hermes_chain_components::traits::types::height::HasHeightType;
use hermes_comet_light_client_components::traits::trace_verification_height::VerificationHeightTracer;
use tendermint::block::Height;

use crate::traits::verification_trace::HasVerificationTrace;

pub struct TraceTendermintVerification;

impl<Client> VerificationHeightTracer<Client> for TraceTendermintVerification
where
    Client: HasHeightType<Height = Height> + HasVerificationTrace,
{
    fn trace_verification_height(
        client: &mut Client,
        target_height: &Height,
        current_height: &Height,
    ) {
        client
            .verification_trace_mut()
            .entry(*target_height)
            .or_default()
            .insert(*current_height);
    }
}