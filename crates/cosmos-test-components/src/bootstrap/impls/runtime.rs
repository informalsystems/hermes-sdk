use eyre::{eyre, Error};
use ibc_relayer_components::runtime::traits::runtime::HasRuntime;
use ibc_relayer_runtime::types::error::TokioRuntimeError;
use ibc_relayer_runtime::types::runtime::TokioRuntimeContext;

use crate::bootstrap::types::bootstrap::CosmosBootstrapContext;

impl HasRuntime for CosmosBootstrapContext {
    type Runtime = TokioRuntimeContext;

    fn runtime_error(e: TokioRuntimeError) -> Error {
        eyre!("runtime error: {}", e)
    }

    fn runtime(&self) -> &TokioRuntimeContext {
        &self.runtime
    }
}
