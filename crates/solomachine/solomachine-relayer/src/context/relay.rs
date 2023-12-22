use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_relayer_runtime::types::runtime::HermesRuntime;
use ibc_relayer::chain::handle::BaseChainHandle;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;

use crate::types::chain::SolomachineChain;

pub struct SolomachineRelay<Chain> {
    pub runtime: HermesRuntime,
    pub src_chain: SolomachineChain<Chain>,
    pub dst_chain: CosmosChain<BaseChainHandle>,
    pub src_client_id: ClientId,
    pub dst_client_id: ClientId,
}

impl<Chain> SolomachineRelay<Chain> {
    pub fn new(
        runtime: HermesRuntime,
        src_chain: SolomachineChain<Chain>,
        dst_chain: CosmosChain<BaseChainHandle>,
        src_client_id: ClientId,
        dst_client_id: ClientId,
    ) -> Self {
        Self {
            runtime,
            src_chain,
            dst_chain,
            src_client_id,
            dst_client_id,
        }
    }
}
