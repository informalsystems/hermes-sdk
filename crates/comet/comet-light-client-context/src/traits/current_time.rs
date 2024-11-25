use cgp::prelude::*;
use tendermint_light_client_verifier::types::Time;

pub trait HasCurrentTime: Async {
    fn current_time(&self) -> Time;
}
