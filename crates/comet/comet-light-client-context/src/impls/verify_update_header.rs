use hermes_comet_light_client_components::traits::{
    HasLightBlockType, HasVerdictType, UpdateHeaderVerifier, UpdateHeaderVerifierComponent,
};
use hermes_comet_light_client_components::types::Verdict;
use hermes_prelude::*;
use tendermint_light_client_verifier::errors::VerificationErrorDetail;
use tendermint_light_client_verifier::types::LightBlock;
use tendermint_light_client_verifier::{Verdict as TendermintVerdict, Verifier};

use crate::traits::current_time::HasCurrentTime;
use crate::traits::verifier::HasVerifier;
use crate::traits::verifier_options::HasVerifierOptions;

pub struct VerifyUpdateHeaderWithProdVerifier;

#[cgp_provider(UpdateHeaderVerifierComponent)]
impl<Client> UpdateHeaderVerifier<Client> for VerifyUpdateHeaderWithProdVerifier
where
    Client: HasLightBlockType<LightBlock = LightBlock>
        + HasVerdictType<Verdict = Verdict>
        + HasVerifier
        + HasVerifierOptions
        + HasCurrentTime
        + CanRaiseAsyncError<VerificationErrorDetail>,
{
    fn verify_update_header(
        client: &Client,
        untrusted_block: &LightBlock,
        trusted_block: &LightBlock,
    ) -> Result<Verdict, Client::Error> {
        let verifier = client.verifier();
        let options = client.verifier_options();
        let current_time = client.current_time();

        let verdict = verifier.verify_update_header(
            untrusted_block.as_untrusted_state(),
            trusted_block.as_trusted_state(),
            options,
            current_time,
        );

        match verdict {
            TendermintVerdict::Success => Ok(Verdict::Success),
            TendermintVerdict::NotEnoughTrust(_) => Ok(Verdict::NotEnoughTrust),
            TendermintVerdict::Invalid(e) => Err(Client::raise_error(e)),
        }
    }
}
