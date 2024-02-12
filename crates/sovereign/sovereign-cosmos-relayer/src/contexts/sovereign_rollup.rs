use cgp_core::HasComponents;
use hermes_sovereign_client_components::sovereign::traits::rollup::json_rpc_client::{
    JsonRpcClientGetter, ProvideJsonRpcClientType,
};
use jsonrpsee::http_client::HttpClient;

pub struct SovereignRollup {
    pub rpc_client: HttpClient,
}

pub struct SovereignRollupComponents;

impl HasComponents for SovereignRollup {
    type Components = SovereignRollupComponents;
}

impl ProvideJsonRpcClientType<SovereignRollup> for SovereignRollupComponents {
    type JsonRpcClient = HttpClient;
}

impl JsonRpcClientGetter<SovereignRollup> for SovereignRollupComponents {
    fn json_rpc_client(rollup: &SovereignRollup) -> &HttpClient {
        &rollup.rpc_client
    }
}
