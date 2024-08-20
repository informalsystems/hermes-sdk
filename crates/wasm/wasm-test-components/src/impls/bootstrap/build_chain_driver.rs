use alloc::collections::BTreeMap;
use core::marker::PhantomData;

use cgp_core::error::CanRaiseError;
use hermes_cosmos_test_components::bootstrap::traits::chain::build_chain_driver::ChainDriverBuilder;
use hermes_cosmos_test_components::bootstrap::traits::types::chain_node_config::HasChainNodeConfigType;
use hermes_cosmos_test_components::bootstrap::traits::types::genesis_config::HasChainGenesisConfigType;
use hermes_cosmos_test_components::chain::types::amount::Amount;
use hermes_cosmos_test_components::chain::types::denom::Denom;
use hermes_cosmos_test_components::chain::types::proposal_status::ProposalStatus;
use hermes_relayer_components::multi::types::index::Index;
use hermes_runtime_components::traits::fs::file_path::HasFilePathType;
use hermes_runtime_components::traits::os::child_process::HasChildProcessType;
use hermes_runtime_components::traits::runtime::HasRuntimeType;
use hermes_test_components::chain::traits::proposal::types::proposal_id::HasProposalIdType;
use hermes_test_components::chain::traits::proposal::types::proposal_status::HasProposalStatusType;
use hermes_test_components::chain::traits::types::amount::HasAmountType;
use hermes_test_components::chain::traits::types::wallet::HasWalletType;
use hermes_test_components::chain_driver::traits::fields::denom_at::{HasDenomAt, StakingDenom};
use hermes_test_components::chain_driver::traits::fields::wallet::{HasWalletAt, ValidatorWallet};
use hermes_test_components::chain_driver::traits::proposal::deposit::CanDepositProposal;
use hermes_test_components::chain_driver::traits::proposal::poll_status::CanPollProposalStatus;
use hermes_test_components::chain_driver::traits::proposal::vote::CanVoteProposal;
use hermes_test_components::chain_driver::traits::types::chain::HasChain;
use hermes_test_components::driver::traits::types::chain_driver::HasChainDriverType;

use crate::traits::bootstrap::client_byte_code::HasWasmClientByteCode;
use crate::traits::chain::upload_client_code::CanUploadWasmClientCode;

pub struct BuildChainDriverAndInitWasmClient<InBuilder>(pub PhantomData<InBuilder>);

impl<Bootstrap, ChainDriver, Chain, Runtime, InBuilder> ChainDriverBuilder<Bootstrap>
    for BuildChainDriverAndInitWasmClient<InBuilder>
where
    Bootstrap: HasRuntimeType<Runtime = Runtime>
        + HasChainDriverType<ChainDriver = ChainDriver, Chain = Chain>
        + HasChainGenesisConfigType
        + HasChainNodeConfigType
        + HasWasmClientByteCode
        + CanRaiseError<Chain::Error>
        + CanRaiseError<ChainDriver::Error>,
    Runtime: HasChildProcessType + HasFilePathType,
    Chain: HasWalletType
        + HasProposalIdType<ProposalId = u64>
        + HasProposalStatusType<ProposalStatus = ProposalStatus>
        + HasAmountType<Amount = Amount, Denom = Denom>
        + CanUploadWasmClientCode,
    ChainDriver: HasChain<Chain = Chain>
        + HasWalletAt<ValidatorWallet, 0>
        + HasDenomAt<StakingDenom, 0>
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

        let chain = chain_driver.chain();

        let validator_wallet = chain_driver.wallet_at(ValidatorWallet, Index::<0>);

        let staking_denom = chain_driver.denom_at(StakingDenom, Index::<0>);

        chain
            .upload_wasm_client_code(
                bootstrap.wasm_client_byte_code(),
                "wasm-client",
                "Wasm Client",
                &Amount {
                    quantity: 20000,
                    denom: staking_denom.clone(),
                },
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
