use cgp::prelude::CanRaiseError;

use hermes_comet_light_client_components::traits::types::light_block::HasLightBlockType;
use hermes_comet_light_client_components::traits::types::verdict::HasVerdictType;
use hermes_comet_light_client_components::traits::verify_update_header::UpdateHeaderVerifier;
use hermes_comet_light_client_components::types::verdict::Verdict;
use tendermint_light_client_verifier::errors::VerificationErrorDetail;
use tendermint_light_client_verifier::types::LightBlock;
use tendermint_light_client_verifier::{Verdict as TendermintVerdict, Verifier};

use crate::traits::current_time::HasCurrentTime;
use crate::traits::verifier::HasVerifier;
use crate::traits::verifier_options::HasVerifierOptions;

pub struct VerifyUpdateHeaderWithProdVerifier;

impl<Client> UpdateHeaderVerifier<Client> for VerifyUpdateHeaderWithProdVerifier
where
    Client: HasLightBlockType<LightBlock = LightBlock>
        + HasVerdictType<Verdict = Verdict>
        + HasVerifier
        + HasVerifierOptions
        + HasCurrentTime
        + CanRaiseError<VerificationErrorDetail>,
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
