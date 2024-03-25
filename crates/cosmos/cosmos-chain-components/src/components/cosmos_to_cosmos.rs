use cgp_core::prelude::*;
use hermes_relayer_components::chain::impls::delegate::queries::client_state::QueryAndDecodeClientStateVia;
use hermes_relayer_components::chain::traits::message_builders::create_client::CreateClientMessageBuilderComponent;
use hermes_relayer_components::chain::traits::queries::client_state::{
    AllClientStatesQuerierComponent, ClientStateQuerierComponent,
};
use prost_types::Any;

use crate::impls::client::create_client_message::BuildCosmosCreateClientMessage;

pub struct CosmosToCosmosComponents;

delegate_components! {
    CosmosToCosmosComponents {
        [
            ClientStateQuerierComponent,
            AllClientStatesQuerierComponent,
        ]:
            QueryAndDecodeClientStateVia<Any>,
        CreateClientMessageBuilderComponent:
            BuildCosmosCreateClientMessage,
    }
}
