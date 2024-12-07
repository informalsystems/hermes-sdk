use core::marker::PhantomData;

use cgp::core::component::UseContext;
use cgp::prelude::*;
use tendermint_light_client_verifier::types::PeerId;

#[cgp_component {
  name: PeerIdGetterComponent,
  provider: PeerIdGetter,
  context: Client,
}]
pub trait HasPeerId: Async {
    fn peer_id(&self) -> &PeerId;
}

impl<Client> PeerIdGetter<Client> for UseContext
where
    Client: Async + HasField<symbol!("peer_id"), Value = PeerId>,
{
    fn peer_id(client: &Client) -> &PeerId {
        client.get_field(PhantomData)
    }
}
