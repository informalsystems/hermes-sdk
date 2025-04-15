use alloc::collections::BTreeMap;
use core::marker::PhantomData;
use core::time::Duration;

use cgp::prelude::*;
use hermes_chain_type_components::traits::{HasAmountType, HasDenomType};
use hermes_cosmos_test_components::bootstrap::traits::{
    ChainDriverBuilder, ChainDriverBuilderComponent, HasChainGenesisConfigType,
    HasChainNodeConfigType,
};
use hermes_cosmos_test_components::chain::types::{Amount, Denom, ProposalStatus, ProposalVote};
use hermes_relayer_components::transaction::traits::CanSendMessagesWithSigner;
use hermes_runtime_components::traits::{
    CanSleep, HasChildProcessType, HasFilePathType, HasRuntime,
};
use hermes_test_components::chain::traits::{
    CanBuildDepositProposalMessage, CanBuildVoteProposalMessage, CanPollProposalStatus,
    HasProposalIdType, HasProposalStatusType, HasProposalVoteType, HasWalletSigner,
};
use hermes_test_components::chain_driver::traits::{
    HasChain, HasChainType, HasDenom, HasWallet, StakingDenom, ValidatorWallet,
};
use hermes_test_components::driver::traits::HasChainDriverType;

use crate::traits::bootstrap::client_byte_code::HasWasmClientByteCode;
use crate::traits::bootstrap::gov_authority::HasGovernanceProposalAuthority;
use crate::traits::chain::upload_client_code::CanUploadWasmClientCode;

pub struct BuildChainDriverAndInitWasmClient<InBuilder>(pub PhantomData<InBuilder>);

#[cgp_provider(ChainDriverBuilderComponent)]
impl<Bootstrap, ChainDriver, Chain, Runtime, InBuilder> ChainDriverBuilder<Bootstrap>
    for BuildChainDriverAndInitWasmClient<InBuilder>
where
    Bootstrap: HasRuntime<Runtime = Runtime>
        + HasChainType<Chain = Chain>
        + HasChainDriverType<ChainDriver = ChainDriver>
        + HasChainGenesisConfigType
        + HasChainNodeConfigType
        + HasWasmClientByteCode
        + HasGovernanceProposalAuthority
        + CanRaiseAsyncError<Chain::Error>,
    Runtime: HasChildProcessType + HasFilePathType + CanSleep,
    Chain: HasWalletSigner
        + HasProposalIdType
        + HasProposalStatusType<ProposalStatus = ProposalStatus>
        + HasProposalVoteType<ProposalVote = ProposalVote>
        + HasAmountType<Amount = Amount>
        + HasDenomType<Denom = Denom>
        + CanUploadWasmClientCode
        + CanUploadWasmClientCode
        + CanPollProposalStatus
        + CanBuildDepositProposalMessage
        + CanBuildVoteProposalMessage
        + CanSendMessagesWithSigner,
    ChainDriver: HasChain<Chain = Chain> + HasWallet<ValidatorWallet> + HasDenom<StakingDenom>,
    InBuilder: ChainDriverBuilder<Bootstrap>,
{
    async fn build_chain_driver(
        bootstrap: &Bootstrap,
        genesis_config: Bootstrap::ChainGenesisConfig,
        chain_node_config: Bootstrap::ChainNodeConfig,
        wallets: BTreeMap<String, Chain::Wallet>,
        chain_process: Vec<Runtime::ChildProcess>,
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

        let validator_wallet = chain_driver.wallet(PhantomData::<ValidatorWallet>);

        let staking_denom = chain_driver.denom(PhantomData::<StakingDenom>);

        let proposal_id = chain
            .upload_wasm_client_code(
                bootstrap.wasm_client_byte_code(),
                "wasm-client",
                "Wasm Client",
                bootstrap.governance_proposal_authority(),
                &Amount {
                    quantity: 1000000,
                    denom: staking_denom.clone(),
                },
            )
            .await
            .map_err(Bootstrap::raise_error)?;

        bootstrap.runtime().sleep(Duration::from_secs(3)).await;

        let status = chain
            .poll_proposal_status(
                &proposal_id,
                &[ProposalStatus::DepositPeriod, ProposalStatus::VotingPeriod],
            )
            .await
            .map_err(Bootstrap::raise_error)?;

        if status == ProposalStatus::DepositPeriod {
            let deposit_message = chain.build_deposit_proposal_message(
                &proposal_id,
                &Amount::new(1000000000, staking_denom.clone()),
            );

            chain
                .send_messages_with_signer(
                    Chain::wallet_signer(validator_wallet),
                    &[deposit_message],
                )
                .await
                .map_err(Bootstrap::raise_error)?;
        }

        chain
            .poll_proposal_status(&proposal_id, &[ProposalStatus::VotingPeriod])
            .await
            .map_err(Bootstrap::raise_error)?;

        {
            let vote_message = chain.build_vote_proposal_message(&proposal_id, &ProposalVote::Yes);

            chain
                .send_messages_with_signer(Chain::wallet_signer(validator_wallet), &[vote_message])
                .await
                .map_err(Bootstrap::raise_error)?;
        }

        chain
            .poll_proposal_status(&proposal_id, &[ProposalStatus::Passed])
            .await
            .map_err(Bootstrap::raise_error)?;

        Ok(chain_driver)
    }
}
