use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;

#[derive_component(ChainTypeComponent, ProvideChainType<Bootstrap>)]
pub trait HasChainType: Async {
    type Chain: HasIbcChainTypes<Self::Counterparty>;

    type Counterparty: Async;
}
