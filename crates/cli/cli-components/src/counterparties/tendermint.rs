use cgp_core::prelude::*;
use hermes_cosmos_client_components::impls::decoders::client_state::DecodeTendermintClientStateProto;
use hermes_cosmos_client_components::impls::types::client_state::ProvideTendermintClientState;
use hermes_relayer_components::chain::traits::types::client_state::{
    ClientStateDecoderComponent, ClientStateTypeComponent,
};

pub struct TendermintCounterparty;

pub struct TendermintCounterpartyComponents;

impl HasComponents for TendermintCounterparty {
    type Components = TendermintCounterpartyComponents;
}

delegate_components! {
    TendermintCounterpartyComponents {
        ClientStateTypeComponent:
            ProvideTendermintClientState,
        ClientStateDecoderComponent:
            DecodeTendermintClientStateProto,
    }
}
