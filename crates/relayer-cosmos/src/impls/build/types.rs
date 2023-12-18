use ibc_relayer::chain::handle::BaseChainHandle;
use ibc_relayer_components::build::traits::birelay::HasBiRelayType;
use ibc_relayer_components::runtime::traits::runtime::HasRuntime;
use ibc_relayer_runtime::types::runtime::TokioRuntimeContext;

use crate::contexts::birelay::CosmosBiRelay;
use crate::contexts::builder::CosmosBuilder;
use crate::types::error::Error;

impl HasBiRelayType for CosmosBuilder {
    type BiRelay = CosmosBiRelay<BaseChainHandle, BaseChainHandle>;

    fn birelay_error(e: Error) -> Error {
        e
    }
}

impl HasRuntime for CosmosBuilder {
    type Runtime = TokioRuntimeContext;

    fn runtime(&self) -> &TokioRuntimeContext {
        &self.runtime
    }
}
