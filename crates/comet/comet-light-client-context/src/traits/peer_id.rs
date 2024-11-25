use cgp::prelude::*;
use tendermint_light_client_verifier::types::PeerId;

pub trait HasPeerId: Async {
    fn peer_id(&self) -> &PeerId;
}
