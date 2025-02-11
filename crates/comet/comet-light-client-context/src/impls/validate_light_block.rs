use core::time::Duration;

use cgp::prelude::*;
use hermes_comet_light_client_components::traits::types::light_block::HasLightBlockType;
use hermes_comet_light_client_components::traits::validate_light_block::{
    LightBlockValidator, LightBlockValidatorComponent,
};
use tendermint::Time;
use tendermint_light_client_verifier::types::LightBlock;

use crate::traits::current_time::HasCurrentTime;
use crate::traits::verifier_options::HasVerifierOptions;

pub struct ValidateTendermintLightBlock;

#[derive(Debug)]
pub struct TrustedStateOutsideTrustingPeriod<'a> {
    pub light_block: &'a LightBlock,
    pub trusting_period: Duration,
}

#[cgp_provider(LightBlockValidatorComponent)]
impl<Client, Mode> LightBlockValidator<Client, Mode> for ValidateTendermintLightBlock
where
    Client: HasLightBlockType<LightBlock = LightBlock>
        + HasVerifierOptions
        + HasCurrentTime
        + for<'a> CanRaiseAsyncError<TrustedStateOutsideTrustingPeriod<'a>>,
{
    fn validate_light_block(
        client: &Client,
        _mode: Mode,
        light_block: &LightBlock,
    ) -> Result<(), Client::Error> {
        let trusting_period = client.verifier_options().trusting_period;
        let now = client.current_time();

        if !is_within_trust_period(light_block, trusting_period, now) {
            Err(Client::raise_error(TrustedStateOutsideTrustingPeriod {
                light_block,
                trusting_period,
            }))
        } else {
            Ok(())
        }
    }
}

pub fn is_within_trust_period(
    light_block: &LightBlock,
    trusting_period: Duration,
    now: Time,
) -> bool {
    let header_time = light_block.signed_header.header.time;
    match now - trusting_period {
        Ok(start) => header_time > start,
        Err(_) => false,
    }
}
