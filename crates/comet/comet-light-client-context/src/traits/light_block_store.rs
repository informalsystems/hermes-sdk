use core::marker::PhantomData;
use std::collections::BTreeMap;

use cgp::core::component::UseContext;
use hermes_comet_light_client_components::types::VerificationStatus;
use hermes_prelude::*;
use tendermint::block::Height;
use tendermint_light_client_verifier::types::LightBlock;

pub type LightBlockStore = BTreeMap<Height, (LightBlock, VerificationStatus)>;

#[cgp_component {
  provider: LightBlockStoreGetter,
  context: Client,
}]
pub trait HasLightBlockStore: Async {
    fn light_block_store(&self) -> &LightBlockStore;

    fn light_block_store_mut(&mut self) -> &mut LightBlockStore;
}

#[cgp_provider(LightBlockStoreGetterComponent)]
impl<Client: Async> LightBlockStoreGetter<Client> for UseFields
where
    Client: HasFieldMut<symbol!("light_block_store"), Value = LightBlockStore>,
{
    fn light_block_store(client: &Client) -> &LightBlockStore {
        client.get_field(PhantomData)
    }

    fn light_block_store_mut(client: &mut Client) -> &mut LightBlockStore {
        client.get_field_mut(PhantomData)
    }
}
