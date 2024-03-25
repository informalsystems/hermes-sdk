use cgp_core::prelude::*;
use hermes_relayer_components::chain::impls::queries::client_state::QueryAndDecodeClientStateVia;
use hermes_relayer_components::chain::traits::queries::client_state::{
    AllClientStatesQuerierComponent, ClientStateQuerierComponent,
};
use prost_types::Any;

pub struct CosmosToCosmosComponents;

delegate_components! {
    CosmosToCosmosComponents {
        [
            ClientStateQuerierComponent,
            AllClientStatesQuerierComponent,
        ]:
            QueryAndDecodeClientStateVia<Any>,
    }
}
