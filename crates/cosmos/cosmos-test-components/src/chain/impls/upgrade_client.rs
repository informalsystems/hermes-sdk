use core::marker::PhantomData;
use core::time::Duration;

use hermes_core::chain_components::traits::{
    CanQueryChainHeight, CanQueryClientStateWithLatestHeight, HasAmountType, HasClientIdType,
    HasClientStateType,
};
use hermes_core::logging_components::traits::CanLog;
use hermes_core::logging_components::types::LevelDebug;
use hermes_core::relayer_components::multi::traits::chain_at::HasChainTypeAt;
use hermes_core::relayer_components::multi::traits::client_id_at::HasClientIdAt;
use hermes_core::relayer_components::transaction::traits::CanSendMessagesWithSigner;
use hermes_cosmos_chain_components::traits::{CosmosMessage, DynCosmosMessage, ToCosmosMessage};
use hermes_cosmos_chain_components::types::TendermintClientState;
use hermes_prelude::*;
use hermes_test_components::chain::traits::{
    CanBuildDepositProposalMessage, CanBuildVoteProposalMessage, CanQueryProposalStatus,
    HasWalletSigner,
};
use hermes_test_components::chain::types::{ProposalStatus, ProposalVote};
use hermes_test_components::chain_driver::traits::{
    CanGenerateRandomAmount, HasChain, HasDenom, HasWallet, StakingDenom, ValidatorWallet,
};
use hermes_test_components::driver::traits::HasChainDriverAt;
use hermes_test_components::test_case::traits::upgrade_client::{
    SetupUpgradeClientTestHandler, SetupUpgradeClientTestHandlerComponent, UpgradeClientHandler,
    UpgradeClientHandlerComponent,
};
use ibc::clients::tendermint::types::TrustThreshold;
use ibc::core::client::types::Height;
use ibc::core::host::types::identifiers::{ChainId, ClientId};
use ibc::cosmos_host::proto::v1beta1::Plan;
use ibc::primitives::proto::Any;
use ibc::primitives::Signer;
use ibc_proto::cosmos::base::v1beta1::Coin;
use ibc_proto::cosmos::gov::v1::MsgSubmitProposal;
use ibc_proto::ibc::core::client::v1::MsgIbcSoftwareUpgrade;

use crate::chain::types::Amount;

const MAX_RETRIES_FOR_PROPOSAL_STATUS: usize = 15;
const WAIT_SECONDS_FOR_PROPOSAL_STATUS: u64 = 1;

pub struct CosmosHandleUpgradeClient;

#[cgp_provider(UpgradeClientHandlerComponent)]
impl<Driver, ChainDriverA, ChainA, ChainB>
    UpgradeClientHandler<Driver, ChainDriverA, ChainA, ChainB> for CosmosHandleUpgradeClient
where
    Driver: HasChainDriverAt<Index<0>, ChainDriver = ChainDriverA> + HasAsyncErrorType,
    ChainDriverA: HasChain<Chain = ChainA>,
    ChainA: HasClientIdType<ChainB, ClientId = ClientId>,
{
    async fn handle_upgrade_client(_driver: &Driver) -> Result<(), Driver::Error> {
        Ok(())
    }
}

pub struct SetupCosmosUpgradeClientTest;

#[cgp_provider(SetupUpgradeClientTestHandlerComponent)]
impl<Driver, ChainDriverA, ChainDriverB, ChainA, ChainB>
    SetupUpgradeClientTestHandler<Driver, ChainDriverA, ChainA, ChainB>
    for SetupCosmosUpgradeClientTest
where
    Driver: HasChainDriverAt<Index<0>, ChainDriver = ChainDriverA>
        + HasChainDriverAt<Index<1>, ChainDriver = ChainDriverB>
        + HasClientIdAt<Index<1>, Index<0>>
        + HasChainTypeAt<Index<0>, Chain = ChainA>
        + HasChainTypeAt<Index<1>, Chain = ChainB>
        + CanLog<LevelDebug>
        + CanRaiseAsyncError<String>,
    ChainDriverA: HasChain<Chain = ChainA>
        + HasWallet<ValidatorWallet>
        + CanGenerateRandomAmount
        + HasDenom<StakingDenom>,
    ChainDriverB: HasChain<Chain = ChainB>,
    ChainA: CanQueryProposalStatus<ProposalId = u64, ProposalStatus = ProposalStatus>
        + CanBuildDepositProposalMessage
        + CanBuildVoteProposalMessage<ProposalVote = ProposalVote>
        + CanQueryChainHeight<Height = Height>
        + HasClientIdType<ChainB, ClientId = ClientId>
        + HasClientStateType<ChainB, ClientState = TendermintClientState>
        + HasWalletSigner
        + HasAmountType<Amount = Amount>
        + CanSendMessagesWithSigner<Message = CosmosMessage>,
    ChainB: CanQueryClientStateWithLatestHeight<ChainA>,
{
    async fn setup_upgrade_client_test(driver: &Driver) -> Result<(), Driver::Error> {
        let chain_driver_a = driver.chain_driver_at(PhantomData::<Index<0>>);

        let chain_a = chain_driver_a.chain();

        let validator_wallet = chain_driver_a.wallet(PhantomData::<ValidatorWallet>);

        let denom_a = chain_driver_a.denom(PhantomData::<StakingDenom>);

        let deposit_amount = chain_driver_a.fixed_amount(11000000, denom_a).await;

        let chain_driver_b = driver.chain_driver_at(PhantomData::<Index<1>>);

        let chain_b = chain_driver_b.chain();

        let client_id_b = driver.client_id_at(PhantomData::<(Index<1>, Index<0>)>);

        let upgrade_height = chain_a.query_chain_height().await.unwrap();

        let mut upgrade_client_state = chain_b
            .query_client_state_with_latest_height(PhantomData, client_id_b)
            .await
            .unwrap();

        // Reset custom fields to zero values
        upgrade_client_state.trusting_period = Duration::from_secs(0);
        upgrade_client_state.trust_level = TrustThreshold::ZERO;
        upgrade_client_state.allow_update.after_expiry = false;
        upgrade_client_state.allow_update.after_misbehaviour = false;
        upgrade_client_state.frozen_height = None;
        upgrade_client_state.max_clock_drift = Duration::from_secs(0);

        // Upgrade the client state
        upgrade_client_state.latest_height = upgrade_height;
        upgrade_client_state.chain_id = ChainId::new("upgraded_chain").unwrap();

        let proposal = IbcSoftwareUpgrade {
            name: "Upgrade for client test".to_owned(),
            height: upgrade_height,
            info: "Chain upgrade plan for client upgrade test".to_owned(),
            upgraded_client_state: upgrade_client_state,
            deposit_amount: deposit_amount.clone(),
        };

        let upgrade_chain_message = proposal.to_cosmos_message();

        chain_a
            .send_messages_with_signer(
                ChainA::wallet_signer(validator_wallet),
                &[upgrade_chain_message],
            )
            .await
            .map_err(|e| {
                Driver::raise_error(format!(
                    "failed to send chain upgrade proposal message: {e:?}"
                ))
            })?;

        let proposal_status = chain_a.query_proposal_status(&1).await.map_err(|e| {
            Driver::raise_error(format!(
                "failed to query proposal status with id `1`: {e:?}"
            ))
        })?;

        if proposal_status == ProposalStatus::DepositPeriod {
            driver
                .log(
                    "chain upgrade proposal `1` is in deposit period",
                    &LevelDebug,
                )
                .await;
            let deposit_message = chain_a.build_deposit_proposal_message(&1, &deposit_amount);

            chain_a
                .send_messages_with_signer(
                    ChainA::wallet_signer(validator_wallet),
                    &[deposit_message],
                )
                .await
                .map_err(|e| Driver::raise_error(format!("{e:?}")))?;
        }

        let mut try_number = 0;
        loop {
            let proposal_status = chain_a.query_proposal_status(&1).await.map_err(|e| {
                Driver::raise_error(format!("failed to query proposal status with id `1`:{e:?}"))
            })?;

            if proposal_status == ProposalStatus::VotingPeriod {
                driver
                    .log(
                        "chain upgrade proposal `1` is in voting period",
                        &LevelDebug,
                    )
                    .await;
                let voting_message = chain_a.build_vote_proposal_message(&1, &ProposalVote::Yes);

                chain_a
                    .send_messages_with_signer(
                        ChainA::wallet_signer(validator_wallet),
                        &[voting_message],
                    )
                    .await
                    .map_err(|e| {
                        Driver::raise_error(format!("failed to send proposal vote message: {e:?}"))
                    })?;

                break;
            }

            if try_number > MAX_RETRIES_FOR_PROPOSAL_STATUS {
                return Err(Driver::raise_error(format!("Chain upgrade proposal failed, expected proposal to be in voting period but is {proposal_status:?}")));
            }
            try_number += 1;

            tokio::time::sleep(Duration::from_secs(WAIT_SECONDS_FOR_PROPOSAL_STATUS)).await;
        }

        try_number = 0;
        loop {
            let proposal_status = chain_a.query_proposal_status(&1).await.map_err(|e| {
                Driver::raise_error(format!("failed to query proposal status with id `1`:{e:?}"))
            })?;

            if proposal_status == ProposalStatus::Passed {
                driver
                    .log("chain upgrade proposal `1` passed", &LevelDebug)
                    .await;
                return Ok(());
            }

            if try_number > MAX_RETRIES_FOR_PROPOSAL_STATUS {
                return Err(Driver::raise_error(format!(
                    "Chain upgrade proposal failed with status {proposal_status:?}"
                )));
            }
            try_number += 1;

            tokio::time::sleep(Duration::from_secs(WAIT_SECONDS_FOR_PROPOSAL_STATUS)).await;
        }
    }
}

#[derive(Clone, Debug)]
struct IbcSoftwareUpgrade {
    pub name: String,
    pub height: Height,
    pub info: String,
    pub upgraded_client_state: TendermintClientState,
    pub deposit_amount: Amount,
}

impl DynCosmosMessage for IbcSoftwareUpgrade {
    fn encode_protobuf(&self, signer: &Signer) -> Any {
        let upgrade_message = MsgIbcSoftwareUpgrade {
            plan: Some(Plan {
                name: self.name.clone(),
                height: (self.height.revision_height() as i64) + 15,
                info: self.info.clone(),
                ..Default::default() // deprecated fields - time & upgraded_client_state
            }),
            upgraded_client_state: Some(Any::from(self.upgraded_client_state.clone())),
            signer: "osmo10d07y265gmmuvt4z0w9aw880jnsr700jjeq4qp".to_owned(), // TODO: don't hard code this
        };

        let proposal_message = MsgSubmitProposal {
            messages: vec![Any::from_msg(&upgrade_message)
                .expect("failed to convert `MsgIbcSoftwareUpgrade` to `Any`")],
            initial_deposit: vec![Coin {
                denom: self.deposit_amount.denom.to_string(),
                amount: self.deposit_amount.quantity.to_string(),
            }],
            proposer: signer.to_string(),
            metadata: "".into(),
            title: self.name.clone(),
            summary: self.info.clone(),
            expedited: false,
        };

        Any::from_msg(&proposal_message).expect("failed to convert `MsgSubmitProposal` to `Any`")
    }
}
