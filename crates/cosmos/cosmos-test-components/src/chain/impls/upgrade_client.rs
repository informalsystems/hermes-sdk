use core::marker::PhantomData;
use core::time::Duration;

use hermes_core::chain_components::impls::CanWaitChainReachHeight;
use hermes_core::chain_components::traits::{
    CanBuildClientUpgradePayload, CanBuildUpdateClientMessage, CanBuildUpdateClientPayload,
    CanQueryChainHeight, CanQueryClientStateWithLatestHeight, CanSendMessages, CanUpgradeClient,
    HasAmountType, HasChainId, HasClientIdType, HasClientStateFields, HasClientStateType,
    HasHeightType,
};
use hermes_core::logging_components::traits::CanLog;
use hermes_core::logging_components::types::LevelDebug;
use hermes_core::relayer_components::multi::traits::chain_at::HasChainTypeAt;
use hermes_core::relayer_components::multi::traits::client_id_at::HasClientIdAt;
use hermes_core::relayer_components::transaction::traits::{
    CanSendMessagesWithSigner, HasDefaultSigner,
};
use hermes_cosmos_chain_components::traits::{
    CosmosMessage, DynCosmosMessage, HasGrpcAddress, ToCosmosMessage,
};
use hermes_cosmos_chain_components::types::TendermintClientState;
use hermes_prelude::*;
use hermes_test_components::chain::traits::{
    CanBuildDepositProposalMessage, CanBuildVoteProposalMessage, CanQueryProposalStatus,
    HasWalletSigner,
};
use hermes_test_components::chain::types::{ProposalStatus, ProposalVote};
use hermes_test_components::chain_driver::traits::{
    CanGenerateRandomAmount, HasChain, HasDenom, HasSetupUpgradeClientTestResultType, HasWallet,
    StakingDenom, ValidatorWallet,
};
use hermes_test_components::driver::traits::HasChainDriverAt;
use hermes_test_components::test_case::traits::upgrade_client::{
    SetupUpgradeClientTestHandler, SetupUpgradeClientTestHandlerComponent, UpgradeClientHandler,
    UpgradeClientHandlerComponent,
};
use http::uri::InvalidUri;
use http::Uri;
use ibc::clients::tendermint::types::TrustThreshold;
use ibc::core::client::types::error::ClientError;
use ibc::core::client::types::Height;
use ibc::core::host::types::error::IdentifierError;
use ibc::core::host::types::identifiers::ChainId;
use ibc::cosmos_host::proto::v1beta1::Plan;
use ibc::primitives::proto::Any;
use ibc::primitives::Signer;
use ibc_proto::cosmos::base::v1beta1::Coin;
use ibc_proto::cosmos::gov::v1::query_client::QueryClient;
use ibc_proto::cosmos::gov::v1::{MsgSubmitProposal, QueryProposalRequest};
use ibc_proto::ibc::core::client::v1::MsgIbcSoftwareUpgrade;
use prost::{DecodeError, Message};
use tonic::transport::Error as TransportError;
use tonic::Status;

use crate::chain::types::Amount;
use crate::chain_driver::impls::CosmosProposalSetupClientUpgradeResult;

const MAX_RETRIES_FOR_PROPOSAL_STATUS: usize = 15;
const WAIT_SECONDS_FOR_PROPOSAL_STATUS: u64 = 1;

pub struct CosmosHandleUpgradeClient;

#[cgp_provider(UpgradeClientHandlerComponent)]
impl<Driver, ChainDriverA, ChainDriverB, ChainA, ChainB>
    UpgradeClientHandler<Driver, ChainDriverA, ChainA, ChainB> for CosmosHandleUpgradeClient
where
    Driver: HasChainDriverAt<Index<0>, ChainDriver = ChainDriverA>
        + HasChainDriverAt<Index<1>, ChainDriver = ChainDriverB>
        + HasClientIdAt<Index<1>, Index<0>>
        + HasChainTypeAt<Index<0>, Chain = ChainA>
        + HasChainTypeAt<Index<1>, Chain = ChainB>
        + CanLog<LevelDebug>
        + CanRaiseAsyncError<ChainA::Error>
        + CanRaiseAsyncError<ChainB::Error>
        + CanRaiseAsyncError<ClientError>,
    ChainDriverA: HasChain<Chain = ChainA>
        + HasSetupUpgradeClientTestResultType<
            SetupUpgradeClientTestResult = CosmosProposalSetupClientUpgradeResult,
        >,
    ChainDriverB: HasChain<Chain = ChainB>,
    ChainA: CanBuildClientUpgradePayload<ChainB>
        + CanWaitChainReachHeight
        + CanQueryChainHeight<Height = Height>
        + CanBuildUpdateClientPayload<ChainB>
        + HasClientStateType<ChainB>
        + HasClientStateFields<ChainB, ChainId = ChainId>
        + HasAsyncErrorType,
    ChainB: CanUpgradeClient<ChainA>
        + CanSendMessagesWithSigner
        + CanQueryClientStateWithLatestHeight<ChainA>
        + CanBuildUpdateClientMessage<ChainA>
        + HasDefaultSigner
        + CanSendMessages
        + HasClientStateType<ChainA>
        + HasAsyncErrorType,
{
    async fn handle_upgrade_client(
        driver: &Driver,
        setup_result: &CosmosProposalSetupClientUpgradeResult,
    ) -> Result<(), Driver::Error> {
        let chain_driver_a = driver.chain_driver_at(PhantomData::<Index<0>>);

        let chain_a = chain_driver_a.chain();

        let chain_driver_b = driver.chain_driver_at(PhantomData::<Index<1>>);

        let chain_b = chain_driver_b.chain();

        let sender_wallet = chain_b.get_default_signer();

        let client_id_b = driver.client_id_at(PhantomData::<(Index<1>, Index<0>)>);

        let latest_height_a = chain_a
            .query_chain_height()
            .await
            .map_err(Driver::raise_error)?;

        // Build upgrade height based on the setup result
        let upgrade_height = Height::new(
            latest_height_a.revision_number(),
            setup_result.height as u64,
        )
        .map_err(Driver::raise_error)?;

        // Must decrement upgrade height since chain halt 1 height before
        let halt_height = upgrade_height.decrement().map_err(Driver::raise_error)?;

        driver
            .log(
                &format!("will wait for chain to halt at height `{halt_height:?}`"),
                &LevelDebug,
            )
            .await;

        // Wait for upgraded chain to halt
        chain_a
            .wait_chain_reach_height(&halt_height)
            .await
            .map_err(Driver::raise_error)?;

        tokio::time::sleep(Duration::from_secs(3)).await;

        // Must update client before upgrading it
        let client_b_state = chain_b
            .query_client_state_with_latest_height(PhantomData, client_id_b)
            .await
            .map_err(Driver::raise_error)?;

        let client_b_state_height = ChainA::client_state_latest_height(&client_b_state);

        let client_b_update_payload = chain_a
            .build_update_client_payload(&client_b_state_height, &upgrade_height, client_b_state)
            .await
            .map_err(Driver::raise_error)?;

        let update_messages = chain_b
            .build_update_client_message(client_id_b, client_b_update_payload)
            .await
            .map_err(Driver::raise_error)?;

        chain_b
            .send_messages(update_messages)
            .await
            .map_err(Driver::raise_error)?;

        driver
            .log(
                &format!("will upgrade client `{client_id_b:?}`"),
                &LevelDebug,
            )
            .await;

        let upgrade_client_payload = chain_a
            .upgrade_client_payload(&upgrade_height)
            .await
            .map_err(Driver::raise_error)?;

        let upgrade_client_message = chain_b
            .upgrade_client_message(client_id_b, &upgrade_client_payload)
            .await
            .map_err(Driver::raise_error)?;

        chain_b
            .send_messages_with_signer(sender_wallet, &[upgrade_client_message])
            .await
            .map_err(Driver::raise_error)?;

        let client_b_state = chain_b
            .query_client_state_with_latest_height(PhantomData, client_id_b)
            .await
            .map_err(Driver::raise_error)?;

        // Assert the client has been upgraded to the new chain ID
        assert_eq!(
            ChainA::client_state_chain_id(&client_b_state),
            setup_result.new_chain_id.clone(),
        );

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
        + CanRaiseAsyncError<ChainA::Error>
        + CanRaiseAsyncError<ChainB::Error>
        + CanRaiseAsyncError<ClientError>
        + CanRaiseAsyncError<DecodeError>
        + CanRaiseAsyncError<IdentifierError>
        + CanRaiseAsyncError<TransportError>
        + CanRaiseAsyncError<InvalidUri>
        + CanRaiseAsyncError<Status>
        + CanRaiseAsyncError<String>
        + CanRaiseAsyncError<&'static str>,
    ChainDriverA: HasChain<Chain = ChainA>
        + HasWallet<ValidatorWallet>
        + CanGenerateRandomAmount
        + HasDenom<StakingDenom>
        + HasSetupUpgradeClientTestResultType<
            SetupUpgradeClientTestResult = CosmosProposalSetupClientUpgradeResult,
        >,
    ChainDriverB: HasChain<Chain = ChainB>,
    ChainA: CanQueryProposalStatus<ProposalId = u64, ProposalStatus = ProposalStatus>
        + CanBuildDepositProposalMessage
        + CanBuildVoteProposalMessage<ProposalVote = ProposalVote>
        + CanQueryChainHeight
        + HasChainId<ChainId = ChainId>
        + HasClientIdType<ChainB>
        + HasClientStateType<ChainB, ClientState = TendermintClientState>
        + HasWalletSigner
        + HasHeightType<Height = Height>
        + HasAmountType<Amount = Amount>
        + HasGrpcAddress
        + CanSendMessagesWithSigner<Message = CosmosMessage>
        + HasAsyncErrorType,
    ChainB: CanQueryClientStateWithLatestHeight<ChainA> + HasAsyncErrorType,
{
    async fn setup_upgrade_client_test(
        driver: &Driver,
    ) -> Result<CosmosProposalSetupClientUpgradeResult, Driver::Error> {
        let chain_driver_a = driver.chain_driver_at(PhantomData::<Index<0>>);

        let chain_a = chain_driver_a.chain();

        let validator_wallet = chain_driver_a.wallet(PhantomData::<ValidatorWallet>);

        let denom_a = chain_driver_a.denom(PhantomData::<StakingDenom>);

        let deposit_amount = chain_driver_a.fixed_amount(11000000, denom_a).await;

        let chain_driver_b = driver.chain_driver_at(PhantomData::<Index<1>>);

        let chain_b = chain_driver_b.chain();

        let client_id_b = driver.client_id_at(PhantomData::<(Index<1>, Index<0>)>);

        let latest_height = chain_a
            .query_chain_height()
            .await
            .map_err(Driver::raise_error)?;

        let mut upgrade_client_state = chain_b
            .query_client_state_with_latest_height(PhantomData, client_id_b)
            .await
            .map_err(Driver::raise_error)?;

        // Get the current chain ID and revision number
        let (chain_name, chain_revision_number) = chain_a
            .chain_id()
            .split_chain_id()
            .map_err(Driver::raise_error)?;

        // Set the upgrade height to the next revision number and revision height to 1
        let upgrade_height =
            Height::new(chain_revision_number + 1, 1).map_err(Driver::raise_error)?;

        // Upgrade the chain ID to bump the revision number by 1
        let upgraded_chain_id_str = format!("{chain_name}-{}", chain_revision_number + 1);
        let upgrade_chain_id = ChainId::new(&upgraded_chain_id_str).map_err(Driver::raise_error)?;

        // Build the plan height, which is 15 blocks after the queried latest height
        let plan_height = Height::new(
            latest_height.revision_number(),
            latest_height.revision_height() + 15,
        )
        .map_err(Driver::raise_error)?;

        // Reset custom fields to zero values
        upgrade_client_state.trusting_period = Duration::from_secs(0);
        upgrade_client_state.trust_level = TrustThreshold::ZERO;
        upgrade_client_state.allow_update.after_expiry = false;
        upgrade_client_state.allow_update.after_misbehaviour = false;
        upgrade_client_state.frozen_height = None;
        upgrade_client_state.max_clock_drift = Duration::from_secs(0);

        // Upgrade the client state
        upgrade_client_state.latest_height = upgrade_height;
        upgrade_client_state.chain_id = upgrade_chain_id.clone();

        let proposal = IbcSoftwareUpgrade {
            name: "Upgrade for client test".to_owned(),
            height: plan_height,
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
            .map_err(Driver::raise_error)?;

        let proposal_status = chain_a
            .query_proposal_status(&1)
            .await
            .map_err(Driver::raise_error)?;

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
                .map_err(Driver::raise_error)?;
        }

        let mut try_number = 0;
        loop {
            let proposal_status = chain_a
                .query_proposal_status(&1)
                .await
                .map_err(Driver::raise_error)?;

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
                    .map_err(Driver::raise_error)?;

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
            let proposal_status = chain_a
                .query_proposal_status(&1)
                .await
                .map_err(Driver::raise_error)?;

            if proposal_status == ProposalStatus::Passed {
                driver
                    .log("chain upgrade proposal `1` passed", &LevelDebug)
                    .await;

                let mut client = QueryClient::connect(
                    Uri::try_from(&chain_a.grpc_address().to_string())
                        .map_err(Driver::raise_error)?,
                )
                .await
                .map_err(Driver::raise_error)?;

                let request = tonic::Request::new(QueryProposalRequest { proposal_id: 1 });

                let response = client
                    .proposal(request)
                    .await
                    .map(|r| r.into_inner())
                    .map_err(Driver::raise_error)?;

                let proposal = response
                    .proposal
                    .ok_or_else(|| Driver::raise_error("proposal not found: `1`"))?;

                let passed_proposal = MsgIbcSoftwareUpgrade::decode(
                    proposal
                        .messages
                        .first()
                        .ok_or_else(|| {
                            Driver::raise_error("queried proposal `1` `messages` is empty")
                        })?
                        .value
                        .as_slice(),
                )
                .map_err(Driver::raise_error)?;

                let upgrade_height = passed_proposal
                    .plan
                    .ok_or_else(|| {
                        Driver::raise_error("query passed proposal `1` doesn't have a `Plan`")
                    })?
                    .height;

                return Ok(CosmosProposalSetupClientUpgradeResult {
                    height: upgrade_height,
                    new_chain_id: upgrade_chain_id,
                });
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
