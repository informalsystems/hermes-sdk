use futures::lock::Mutex;
use hermes_cosmos_client_components::traits::rpc_client::HasRpcClient;
use hermes_relayer_runtime::types::runtime::HermesRuntime;
use ibc_relayer::chain::cosmos::types::config::TxConfig;
use ibc_relayer::keyring::Secp256k1KeyPair;
use tendermint_rpc::HttpClient;

pub struct CosmosTxContext {
    pub tx_config: TxConfig,
    pub rpc_client: HttpClient,
    pub key_entry: Secp256k1KeyPair,
    pub nonce_mutex: Mutex<()>,
    pub runtime: HermesRuntime,
}

impl CosmosTxContext {
    pub fn new(
        tx_config: TxConfig,
        rpc_client: HttpClient,
        key_entry: Secp256k1KeyPair,
        runtime: HermesRuntime,
    ) -> Self {
        Self {
            tx_config,
            rpc_client,
            key_entry,
            nonce_mutex: Mutex::new(()),
            runtime,
        }
    }
}

impl HasRpcClient for CosmosTxContext {
    fn rpc_client(&self) -> &HttpClient {
        &self.rpc_client
    }

    fn rpc_address(&self) -> &tendermint_rpc::Url {
        &self.tx_config.rpc_address
    }
}
