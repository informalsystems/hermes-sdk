use core::marker::PhantomData;

use cgp::core::component::UseContext;
use hermes_prelude::*;
use tendermint_light_client_verifier::types::PeerId;

#[cgp_component {
  provider: PeerIdGetter,
  context: Client,
}]
pub trait HasPeerId: Async {
    fn peer_id(&self) -> &PeerId;
}

#[cgp_provider(PeerIdGetterComponent)]
impl<Client> PeerIdGetter<Client> for UseFields
where
    Client: Async + HasField<symbol!("peer_id"), Value = PeerId>,
{
    fn peer_id(client: &Client) -> &PeerId {
        client.get_field(PhantomData)
    }
}
