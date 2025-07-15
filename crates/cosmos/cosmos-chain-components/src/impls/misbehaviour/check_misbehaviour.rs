use core::marker::PhantomData;

use hermes_comet_light_client_components::traits::{CanDetectMisbehaviour, CanFetchLightBlock};
use hermes_comet_light_client_context::contexts::light_client::CometLightClient;
use hermes_core::chain_components::traits::{
    CanExtractFromEvent, CanQueryChainHeight, HasChainId, HasClientStateType, HasEventType,
    HasEvidenceType, HasUpdateClientEvent, MisbehaviourChecker, MisbehaviourCheckerComponent,
};
use hermes_core::logging_components::traits::CanLog;
use hermes_core::logging_components::types::{LevelDebug, LevelError, LevelWarn};
use hermes_error::HermesError;
use hermes_prelude::*;
use ibc::core::client::types::Height;
use ibc_client_tendermint::types::error::TendermintClientError;
use tendermint::block::Height as TendermintHeight;
use tendermint::error::Error as TendermintError;
use tendermint::validator::Set;
use tendermint_light_client_verifier::types::{LightBlock, ValidatorSet};
use tendermint_rpc::{Client, Error as TendermintRpcError, Paging};

use crate::traits::HasRpcClient;
use crate::types::{CosmosUpdateClientEvent, TendermintClientState};

pub struct CheckTendermintMisbehaviour;

#[cgp_provider(MisbehaviourCheckerComponent)]
impl<Chain, Counterparty> MisbehaviourChecker<Chain, Counterparty> for CheckTendermintMisbehaviour
where
    Chain: HasEvidenceType
        + HasEventType
        + HasRpcClient
        + HasChainId
        + HasClientStateType<Counterparty, ClientState = TendermintClientState>
        + CanQueryChainHeight<Height = Height>
        + HasUpdateClientEvent<UpdateClientEvent = CosmosUpdateClientEvent>
        + CanExtractFromEvent<Chain::UpdateClientEvent>
        + CanLog<LevelDebug>
        + CanLog<LevelWarn>
        + CanLog<LevelError>
        + CanRaiseAsyncError<TendermintError>
        + CanRaiseAsyncError<TendermintRpcError>
        + CanRaiseAsyncError<TendermintClientError>
        + CanRaiseAsyncError<HermesError>
        + CanRaiseAsyncError<String>,
{
    async fn check_misbehaviour(
        chain: &Chain,
        update_event: &Chain::Event,
        client_state: &Chain::ClientState,
    ) -> Result<Option<Chain::Evidence>, Chain::Error> {
        let update_client_event = chain
            .try_extract_from_event(PhantomData, update_event)
            .ok_or_else(|| {
                Chain::raise_error(format!("missing update client event from {update_event:?}"))
            })?;

        // TODO: Fix this
        let update_header = update_client_event.header.unwrap();

        let latest_height = chain.query_chain_height().await?;

        let rpc_client = chain.rpc_client().clone();

        let tendermint_latest_height = TendermintHeight::try_from(latest_height.revision_height())
            .map_err(Chain::raise_error)?;

        let status = rpc_client.status().await.map_err(Chain::raise_error)?;

        let current_time = status.sync_info.latest_block_time;
        let peer_id = status.node_info.id;

        let light_client_options = TendermintClientState::from(client_state.clone())
            .as_light_client_options()
            .map_err(Chain::raise_error)?;

        let light_client = CometLightClient::new(
            chain.chain_id().to_string(),
            current_time,
            peer_id,
            rpc_client.clone(),
            light_client_options,
        );

        let signed_header_from_event = update_header.signed_header.unwrap();

        let next_validator_height =
            (signed_header_from_event.clone().header.unwrap().height + 1) as u32;
        let next_validator_proposer_address = signed_header_from_event
            .clone()
            .header
            .unwrap()
            .proposer_address
            .try_into()
            .unwrap();

        let next_validators = rpc_client
            .validators(next_validator_height, Paging::All)
            .await
            .unwrap()
            .validators;

        let next_validator_set =
            ValidatorSet::with_proposer(next_validators, next_validator_proposer_address)
                .map_err(Chain::raise_error)?;

        let target_block: LightBlock = LightBlock {
            signed_header: signed_header_from_event.clone().clone().try_into().unwrap(),
            validators: update_header
                .validator_set
                .unwrap()
                .clone()
                .try_into()
                .unwrap(),
            next_validators: next_validator_set,
            provider: peer_id,
        };

        let trusted_block = light_client
            .fetch_light_block(&tendermint_latest_height)
            .await
            .map_err(Chain::raise_error)?;

        let event_trusted_validator_set: Set = update_header
            .trusted_validators
            .unwrap()
            .try_into()
            .unwrap();

        if trusted_block.validators.hash() != event_trusted_validator_set.hash() {
            return Err(Chain::raise_error(format!(
                "mismatch between the trusted validator set of the update \
                header ({}) and that of the trusted block that was fetched ({}), \
                aborting misbehaviour detection.",
                trusted_block.validators.hash(),
                event_trusted_validator_set.hash()
            )));
        }

        let _maybe_divergence = light_client
            .detect(&target_block, &trusted_block)
            .await
            .map_err(Chain::raise_error)?;

        Ok(None)
    }
}
