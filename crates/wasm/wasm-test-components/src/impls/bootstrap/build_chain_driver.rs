use core::marker::PhantomData;
use core::time::Duration;

use alloc::collections::BTreeMap;
use cgp_core::CanRaiseError;
use hermes_cosmos_test_components::bootstrap::traits::chain::build_chain_driver::ChainDriverBuilder;
use hermes_cosmos_test_components::bootstrap::traits::types::chain_node_config::HasChainNodeConfigType;
use hermes_cosmos_test_components::bootstrap::traits::types::genesis_config::HasChainGenesisConfigType;
use hermes_cosmos_test_components::chain::types::amount::Amount;
use hermes_cosmos_test_components::chain::types::denom::Denom;
use hermes_cosmos_test_components::chain::types::proposal_status::ProposalStatus;
use hermes_runtime_components::traits::fs::file_path::HasFilePathType;
use hermes_runtime_components::traits::os::child_process::HasChildProcessType;
use hermes_runtime_components::traits::runtime::HasRuntimeType;
use hermes_runtime_components::traits::sleep::CanSleep;
use hermes_test_components::chain::traits::proposal::types::proposal_id::HasProposalIdType;
use hermes_test_components::chain::traits::proposal::types::proposal_status::HasProposalStatusType;
use hermes_test_components::chain::traits::types::amount::HasAmountType;
use hermes_test_components::chain::traits::types::wallet::HasWalletType;
use hermes_test_components::chain_driver::traits::fields::denom_at::{HasDenomAt, StakingDenom};
use hermes_test_components::chain_driver::traits::fields::wallet::{HasWalletAt, ValidatorWallet};
use hermes_test_components::chain_driver::traits::proposal::deposit::CanDepositProposal;
use hermes_test_components::chain_driver::traits::proposal::poll_status::CanPollProposalStatus;
use hermes_test_components::chain_driver::traits::proposal::vote::CanVoteProposal;
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;
use hermes_test_components::driver::traits::types::chain_driver::HasChainDriverType;
use hermes_test_components::types::index::Index;

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
        + CanRaiseError<ChainDriver::Error>,
    Runtime: HasChildProcessType + HasFilePathType + CanSleep,
    Chain: HasWalletType
        + HasProposalIdType<ProposalId = u64>
        + HasProposalStatusType<ProposalStatus = ProposalStatus>
        + HasAmountType<Amount = Amount, Denom = Denom>,
    ChainDriver: HasChainType<Chain = Chain>
        + HasWalletAt<ValidatorWallet, 0>
        + HasDenomAt<StakingDenom, 0>
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

        bootstrap.runtime().sleep(Duration::from_secs(1)).await;

        let validator_wallet = chain_driver.wallet_at(ValidatorWallet, Index::<0>);

        let staking_denom = chain_driver.denom_at(StakingDenom, Index::<0>);

        chain_driver
            .upload_wasm_client_code(
                bootstrap.wasm_client_code_path(),
                "wasm-client",
                "Wasm Client",
                validator_wallet,
            )
            .await
            .map_err(Bootstrap::raise_error)?;

        chain_driver
            .poll_proposal_status(&1, &ProposalStatus::DepositPeriod)
            .await
            .map_err(Bootstrap::raise_error)?;

        chain_driver
            .deposit_proposal(
                &1,
                &Amount::new(100000000, staking_denom.clone()),
                validator_wallet,
            )
            .await
            .map_err(Bootstrap::raise_error)?;

        chain_driver
            .poll_proposal_status(&1, &ProposalStatus::VotingPeriod)
            .await
            .map_err(Bootstrap::raise_error)?;

        chain_driver
            .vote_proposal(&1, validator_wallet)
            .await
            .map_err(Bootstrap::raise_error)?;

        chain_driver
            .poll_proposal_status(&1, &ProposalStatus::Passed)
            .await
            .map_err(Bootstrap::raise_error)?;

        Ok(chain_driver)
    }
}
