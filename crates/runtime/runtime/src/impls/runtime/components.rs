use cgp::prelude::*;
use hermes_tokio_runtime_components::components::parallel::*;
use hermes_tokio_runtime_components::with_tokio_parallel_runtime_components;

use crate::types::runtime::HermesRuntime;

pub struct HermesRuntimeComponents;

impl HasComponents for HermesRuntime {
    type Components = HermesRuntimeComponents;
}

with_tokio_parallel_runtime_components! {
    | Components | {
        delegate_components! {
            HermesRuntimeComponents {
                Components: TokioParallelRuntimeComponents,
            }
        }
    }
}
