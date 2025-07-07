use alloc::collections::BTreeMap;
use core::marker::PhantomData;
use core::time::Duration;

use hermes_core::chain_components::traits::HasAddressType;
use hermes_core::chain_type_components::traits::{HasAmountType, HasDenomType};
use hermes_core::relayer_components::transaction::traits::{
    CanSendMessagesWithSigner, HasDefaultSigner,
};
use hermes_core::runtime_components::traits::{
    CanSleep, CanWriteStringToFile, HasChildProcessType, HasFilePathType, HasRuntime,
};
use hermes_core::test_components::chain::traits::{
    CanBuildDepositProposalMessage, CanBuildVoteProposalMessage, CanPollProposalStatus,
    HasProposalIdType, HasProposalStatusType, HasProposalVoteType, HasWalletSigner,
};
use hermes_core::test_components::chain_driver::traits::{
    HasChain, HasChainType, HasDenom, HasWallet, StakingDenom, ValidatorWallet,
};
use hermes_core::test_components::driver::traits::HasChainDriverType;
use hermes_cosmos_chain_components::impls::WasmAccessConfig;
use hermes_cosmos_chain_components::types::{HasWasmAccessType, Secp256k1KeyPair};
use hermes_cosmos_test_components::bootstrap::traits::{
    ChainDriverBuilder, ChainDriverBuilderComponent, HasChainGenesisConfigType,
    HasChainNodeConfigType, HasChainStoreDir,
};
use hermes_cosmos_test_components::chain::types::{Amount, Denom};
use hermes_prelude::*;
use hermes_test_components::chain::types::{ProposalStatus, ProposalVote};
use hermes_wasm_chain_components::traits::{CanInstantiateWasmContract, CanUploadWasmContract};
use ibc::primitives::Signer;

use crate::traits::bootstrap::{
    HasGovernanceProposalAuthority, HasWasmAdditionalByteCode, HasWasmClientByteCode,
};
use crate::traits::chain::CanUploadWasmClientCode;

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
        + HasWasmAdditionalByteCode
        + HasChainStoreDir
        + HasGovernanceProposalAuthority
        + CanRaiseAsyncError<Chain::Error>
        + CanRaiseAsyncError<Runtime::Error>,
    Runtime: HasChildProcessType + HasFilePathType + CanSleep + CanWriteStringToFile,
    Chain: HasWalletSigner
        + HasProposalIdType
        + HasDefaultSigner<Signer = Secp256k1KeyPair>
        + HasProposalStatusType<ProposalStatus = ProposalStatus>
        + HasProposalVoteType<ProposalVote = ProposalVote>
        + HasAmountType<Amount = Amount>
        + HasDenomType<Denom = Denom>
        + CanUploadWasmClientCode
        + CanUploadWasmContract
        + CanInstantiateWasmContract
        + HasWasmAccessType<WasmAccess = WasmAccessConfig>
        + CanPollProposalStatus
        + CanBuildDepositProposalMessage
        + CanBuildVoteProposalMessage
        + CanSendMessagesWithSigner
        + HasAddressType<Address = String>,
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

        let chain_home_dir = bootstrap.chain_store_dir();

        // Write the wallet secret as a file so that a tester can use it during manual tests
        let wasm_addresses_file = Runtime::join_file_path(
            chain_home_dir,
            &Runtime::file_path_from_string("wasm-addresses.env"),
        );

        let mut lines = vec![];

        let sender: Signer = chain.get_default_signer().account().into();

        // Upload and instantiate additional Wasm contracts
        for additional_wasm_code in bootstrap.wasm_additional_byte_codes().iter() {
            // TODO: Set correct access type
            let code_id = chain
                .upload_wasm_contract(
                    additional_wasm_code.as_slice(),
                    &sender.as_ref().to_string(),
                    &WasmAccessConfig::Everybody,
                )
                .await
                .map_err(Bootstrap::raise_error)?;
            let contract_address = chain
                .instantiate_wasm_contract(
                    &sender.as_ref().to_string(),
                    bootstrap.governance_proposal_authority(),
                    b"{}".to_vec().as_slice(),
                    code_id,
                    staking_denom,
                )
                .await
                .map_err(Bootstrap::raise_error)?;
            lines.push(format!("WASM_ADDRESS_{code_id}={contract_address}"));
        }

        let wasm_addresses = lines.join("\n");

        bootstrap
            .runtime()
            .write_string_to_file(&wasm_addresses_file, &wasm_addresses)
            .await
            .map_err(Bootstrap::raise_error)?;

        Ok(chain_driver)
    }
}
