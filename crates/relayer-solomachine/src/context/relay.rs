use ibc_relayer::chain::handle::BaseChainHandle;
use ibc_relayer_all_in_one::one_for_all::types::chain::OfaChainWrapper;
use ibc_relayer_cosmos::contexts::chain::CosmosChain;
use ibc_relayer_runtime::types::runtime::TokioRuntimeContext;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;

use crate::types::batch::CosmosBatchSender;
use crate::types::chain::SolomachineChain;

pub struct SolomachineRelay<Chain> {
    pub runtime: TokioRuntimeContext,
    pub src_chain: OfaChainWrapper<SolomachineChain<Chain>>,
    pub dst_chain: OfaChainWrapper<CosmosChain<BaseChainHandle>>,
    pub src_client_id: ClientId,
    pub dst_client_id: ClientId,
    //pub src_chain_message_batch_sender: SolomachineBatchSender,
    pub dst_chain_message_batch_sender: CosmosBatchSender,
}

impl<Chain> SolomachineRelay<Chain> {
    pub fn new(
        runtime: TokioRuntimeContext,
        src_chain: OfaChainWrapper<SolomachineChain<Chain>>,
        dst_chain: OfaChainWrapper<CosmosChain<BaseChainHandle>>,
        src_client_id: ClientId,
        dst_client_id: ClientId,
        //src_chain_message_batch_sender: SolomachineBatchSender,
        dst_chain_message_batch_sender: CosmosBatchSender,
    ) -> Self {
        Self {
            runtime,
            src_chain,
            dst_chain,
            src_client_id,
            dst_client_id,
            //src_chain_message_batch_sender,
            dst_chain_message_batch_sender,
        }
    }
}
