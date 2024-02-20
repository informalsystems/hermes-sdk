use cgp_core::delegate_all;
use cgp_core::prelude::*;
use cgp_core::ErrorRaiserComponent;
use cgp_core::ErrorTypeComponent;
use cgp_core::HasComponents;
use cgp_error_eyre::ProvideEyreError;
use cgp_error_eyre::RaiseDebugError;
use hermes_relayer_components::runtime::traits::runtime::RuntimeTypeComponent;
use hermes_relayer_runtime::impls::types::runtime::ProvideTokioRuntimeType;
use hermes_sovereign_client_components::sovereign::traits::rollup::json_rpc_client::{
    JsonRpcClientGetter, ProvideJsonRpcClientType,
};
use hermes_sovereign_test_components::rollup::components::IsSovereignRollupTestComponent;
use hermes_sovereign_test_components::rollup::components::SovereignRollupTestComponents;
use hermes_test_components::chain::traits::queries::balance::CanQueryBalance;
use jsonrpsee::http_client::HttpClient;

pub struct SovereignRollup {
    pub rpc_client: HttpClient,
}

pub struct SovereignRollupComponents;

impl HasComponents for SovereignRollup {
    type Components = SovereignRollupComponents;
}

delegate_all!(
    IsSovereignRollupTestComponent,
    SovereignRollupTestComponents,
    SovereignRollupComponents,
);

delegate_components! {
    SovereignRollupComponents {
        ErrorTypeComponent:
            ProvideEyreError,
        ErrorRaiserComponent:
            RaiseDebugError,
        RuntimeTypeComponent:
            ProvideTokioRuntimeType,
    }
}

impl ProvideJsonRpcClientType<SovereignRollup> for SovereignRollupComponents {
    type JsonRpcClient = HttpClient;
}

impl JsonRpcClientGetter<SovereignRollup> for SovereignRollupComponents {
    fn json_rpc_client(rollup: &SovereignRollup) -> &HttpClient {
        &rollup.rpc_client
    }
}

pub trait CheckSovereignRollupImpls: CanQueryBalance {}
