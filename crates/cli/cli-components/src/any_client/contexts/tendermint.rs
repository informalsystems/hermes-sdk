use cgp_core::prelude::*;
use hermes_cosmos_client_components::impls::decoders::client_state::DecodeTendermintClientStateProto;
use hermes_cosmos_client_components::impls::types::client_state::ProvideTendermintClientState;
use hermes_relayer_components::chain::traits::types::client_state::{
    ClientStateDecoderComponent, ClientStateTypeComponent,
};

pub struct TendermintChain;

pub struct TendermintChainComponents;

impl HasComponents for TendermintChain {
    type Components = TendermintChainComponents;
}

delegate_components! {
    TendermintChainComponents {
        ClientStateTypeComponent:
            ProvideTendermintClientState,
        ClientStateDecoderComponent:
            DecodeTendermintClientStateProto,
    }
}
