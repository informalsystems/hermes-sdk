use core::marker::PhantomData;

use alloc::collections::BTreeMap;
use cgp_core::prelude::HasErrorType;
use hermes_cosmos_test_components::bootstrap::traits::chain::build_chain_driver::ChainDriverBuilder;
use hermes_cosmos_test_components::bootstrap::traits::types::chain_node_config::HasChainNodeConfigType;
use hermes_cosmos_test_components::bootstrap::traits::types::genesis_config::HasChainGenesisConfigType;
use hermes_runtime_components::traits::fs::file_path::HasFilePathType;
use hermes_runtime_components::traits::os::child_process::HasChildProcessType;
use hermes_runtime_components::traits::runtime::HasRuntimeType;
use hermes_test_components::chain::traits::proposal::types::proposal_id::HasProposalIdType;
use hermes_test_components::chain::traits::proposal::types::proposal_status::HasProposalStatusType;
use hermes_test_components::chain::traits::types::amount::HasAmountType;
use hermes_test_components::chain::traits::types::wallet::HasWalletType;
use hermes_test_components::chain_driver::traits::proposal::deposit::CanDepositProposal;
use hermes_test_components::chain_driver::traits::proposal::poll_status::CanPollProposalStatus;
use hermes_test_components::chain_driver::traits::proposal::vote::CanVoteProposal;
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;
use hermes_test_components::driver::traits::types::chain_driver::HasChainDriverType;

use crate::traits::bootstrap::client_code_path::HasWasmClientCodePath;
use crate::traits::chain_driver::upload_client_code::CanUploadWasmClientCode;

pub struct BuildChainDriverAndInitWasmClient<InBuilder>(pub PhantomData<InBuilder>);

impl<Bootstrap, ChainDriver, Chain, Runtime, InBuilder> ChainDriverBuilder<Bootstrap>
    for BuildChainDriverAndInitWasmClient<InBuilder>
where
    Bootstrap: HasRuntimeType<Runtime = Runtime>
        + HasChainDriverType<ChainDriver = ChainDriver, Chain = Chain>
        + HasChainGenesisConfigType
        + HasChainNodeConfigType
        + HasWasmClientCodePath
        + HasErrorType,
    Runtime: HasChildProcessType + HasFilePathType,
    Chain: HasWalletType + HasAmountType + HasProposalIdType + HasProposalStatusType,
    ChainDriver: HasChainType<Chain = Chain>
        + CanUploadWasmClientCode
        + CanPollProposalStatus
        + CanDepositProposal
        + CanVoteProposal,
    ChainDriver::Runtime: HasFilePathType<FilePath = Runtime::FilePath>,
    InBuilder: ChainDriverBuilder<Bootstrap>,
{
    async fn build_chain_driver(
        bootstrap: &Bootstrap,
        genesis_config: Bootstrap::ChainGenesisConfig,
        chain_node_config: Bootstrap::ChainNodeConfig,
        wallets: BTreeMap<String, Chain::Wallet>,
        chain_process: Runtime::ChildProcess,
    ) -> Result<Bootstrap::ChainDriver, Bootstrap::Error> {
        let chain_driver = InBuilder::build_chain_driver(
            bootstrap,
            genesis_config,
            chain_node_config,
            wallets,
            chain_process,
        )
        .await?;

        todo!()
    }
}
