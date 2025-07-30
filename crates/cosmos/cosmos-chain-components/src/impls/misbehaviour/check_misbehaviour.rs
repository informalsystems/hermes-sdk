//use core::marker::PhantomData;

use cgp::extra::runtime::HasRuntime;
use hermes_comet_light_client_components::traits::{CanDetectMisbehaviour, CanFetchLightBlock};
use hermes_comet_light_client_context::contexts::light_client::CometLightClient;
use hermes_core::chain_components::traits::{
    HasChainId, HasClientStateType, HasEvidenceType, HasUpdateClientEvent, MisbehaviourChecker,
    MisbehaviourCheckerComponent,
};
use hermes_core::logging_components::traits::CanLog;
use hermes_core::logging_components::types::{LevelDebug, LevelWarn};
use hermes_core::runtime_components::traits::CanSleep;
use hermes_error::HermesError;
use hermes_prelude::*;
use ibc::core::client::types::Height;
use ibc::core::host::types::error::DecodingError;
use ibc_client_tendermint::types::error::TendermintClientError;
use ibc_client_tendermint::types::proto::v1::{Header, Misbehaviour};
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
    Chain: HasRpcClient
        + HasRuntime
        + HasChainId
        + HasClientStateType<Counterparty>
        + CanLog<LevelDebug>
        + CanLog<LevelWarn>
        + CanRaiseAsyncError<TendermintError>
        + CanRaiseAsyncError<TendermintRpcError>
        + CanRaiseAsyncError<TendermintClientError>
        + CanRaiseAsyncError<HermesError>
        + CanRaiseAsyncError<DecodingError>
        + CanRaiseAsyncError<&'static str>,
    Counterparty: HasEvidenceType<Evidence = Misbehaviour>
        + HasUpdateClientEvent<UpdateClientEvent = CosmosUpdateClientEvent>,
    TendermintClientState: From<Chain::ClientState>,
    Chain::Runtime: CanSleep,
{
    async fn check_misbehaviour(
        chain: &Chain,
        update_client_event: &Counterparty::UpdateClientEvent,
        client_state: &Chain::ClientState,
    ) -> Result<Option<Counterparty::Evidence>, Chain::Error> {
        let event_header = update_client_event.header.clone();

        let event_signed_header = event_header.signed_header.ok_or_else(|| {
            Chain::raise_error("`signed_header` missing from `Header` in Update Client event")
        })?;

        let event_signed_header_header = event_signed_header.header.clone().ok_or_else(|| {
            Chain::raise_error("`header` missing from `SignedHeader` in Update Client event")
        })?;

        let event_trusted_validator_set: Set = event_header
            .trusted_validators
            .ok_or_else(|| {
                Chain::raise_error(
                    "`trusted_validators` missing from `Header` in Update Client event",
                )
            })?
            .try_into()
            .map_err(Chain::raise_error)?;

        let raw_trusted_height = event_header.trusted_height.ok_or_else(|| {
            Chain::raise_error("`trusted_height` missing from `Header` in Update Client event")
        })?;

        let trusted_height = Height::try_from(raw_trusted_height).map_err(Chain::raise_error)?;
        let tm_trusted_height = TendermintHeight::try_from(trusted_height.revision_height())
            .map_err(Chain::raise_error)?;

        let tm_client_state = TendermintClientState::from(client_state.clone());

        let rpc_client = chain.rpc_client().clone();

        let status = rpc_client.status().await.map_err(Chain::raise_error)?;

        let current_time = status.sync_info.latest_block_time;
        let peer_id = status.node_info.id;

        let light_client_options = tm_client_state
            .as_light_client_options()
            .map_err(Chain::raise_error)?;

        let light_client = CometLightClient::new(
            chain.chain_id().to_string(),
            current_time,
            peer_id,
            rpc_client.clone(),
            light_client_options,
        );

        let next_validator_height = (event_signed_header_header.height + 1) as u32;

        let next_validator_proposer_address = event_signed_header_header
            .clone()
            .proposer_address
            .try_into()
            .map_err(Chain::raise_error)?;

        let next_validators = rpc_client
            .validators(next_validator_height, Paging::All)
            .await
            .map_err(Chain::raise_error)?
            .validators;

        let next_validator_set =
            ValidatorSet::with_proposer(next_validators, next_validator_proposer_address)
                .map_err(Chain::raise_error)?;

        let target_block: LightBlock = LightBlock {
            signed_header: event_signed_header.try_into().map_err(Chain::raise_error)?,
            validators: event_trusted_validator_set.clone(),
            next_validators: next_validator_set,
            provider: peer_id,
        };

        let trusted_block = light_client
            .fetch_light_block(&tm_trusted_height.increment())
            .await
            .map_err(Chain::raise_error)?;

        // Required to avoid bad witness error
        chain
            .runtime()
            .sleep(core::time::Duration::from_secs(1))
            .await;

        if trusted_block.validators.hash() != event_trusted_validator_set.hash() {
            chain
                .log(
                    &format!(
                        "validator hash mismatch (trusted: {}, header: {}), continuing...",
                        trusted_block.validators.hash(),
                        event_trusted_validator_set.hash()
                    ),
                    &LevelWarn,
                )
                .await;
        }

        let maybe_divergence = light_client
            .detect(&target_block, &trusted_block)
            .await
            .map_err(Chain::raise_error)?;

        match maybe_divergence {
            Some(divergence) => {
                chain
                    .log(
                        "Found divergence while checking for misbehaviour",
                        &LevelDebug,
                    )
                    .await;
                let supporting = divergence
                    .evidence
                    .witness_trace
                    .into_vec()
                    .into_iter()
                    .filter(|lb| {
                        lb.height() != target_block.height() && lb.height() != tm_trusted_height
                    })
                    .collect::<Vec<LightBlock>>();

                let trusted_validator_set = light_client
                    .fetch_light_block(&tm_trusted_height.increment())
                    .await
                    .map_err(Chain::raise_error)?
                    .validators;

                let mut supporting_headers = Vec::with_capacity(supporting.len());

                let mut current_trusted_height = trusted_height;
                let mut current_trusted_validators = trusted_validator_set.clone();

                for support in supporting {
                    let header = Header {
                        signed_header: Some(support.signed_header.clone().into()),
                        validator_set: Some(support.validators.into()),
                        trusted_height: Some(current_trusted_height.into()),
                        trusted_validators: Some(current_trusted_validators.into()),
                    };

                    // This header is now considered to be the currently trusted header
                    current_trusted_height = header
                        .trusted_height
                        .ok_or_else(|| {
                            Chain::raise_error("`trusted_height` missing from support `Header`")
                        })?
                        .try_into()
                        .map_err(Chain::raise_error)?;

                    let next_height = TendermintHeight::try_from(
                        header
                            .trusted_height
                            .ok_or_else(|| {
                                Chain::raise_error("`trusted_height` missing from support `Header`")
                            })?
                            .revision_height
                            + 1,
                    )
                    .map_err(Chain::raise_error)?;

                    // Therefore we can now trust the next validator set, see NOTE above.
                    current_trusted_validators = light_client
                        .fetch_light_block(&next_height)
                        .await
                        .map_err(Chain::raise_error)?
                        .validators;

                    supporting_headers.push(header);
                }

                // a) Set the trusted height of the target header to the height of the previous
                // supporting header if any, or to the initial trusting height otherwise.
                //
                // b) Set the trusted validators of the target header to the validators of the successor to
                // the last supporting header if any, or to the initial trusted validators otherwise.
                let (latest_trusted_height, latest_trusted_validator_set) =
                    match supporting_headers.last() {
                        Some(prev_header) => {
                            let prev_height = TendermintHeight::try_from(
                                prev_header
                                    .trusted_height
                                    .ok_or_else(|| {
                                        Chain::raise_error(
                                            "`trusted_height` missing from previous `Header`",
                                        )
                                    })?
                                    .revision_height
                                    + 1,
                            )
                            .map_err(Chain::raise_error)?;
                            let prev_succ = light_client
                                .fetch_light_block(&prev_height)
                                .await
                                .map_err(Chain::raise_error)?;
                            (
                                prev_header.trusted_height.ok_or_else(|| {
                                    Chain::raise_error(
                                        "`trusted_height` missing from previous `Header`",
                                    )
                                })?,
                                prev_succ.validators,
                            )
                        }
                        None => (trusted_height.into(), trusted_validator_set),
                    };

                #[allow(deprecated)]
                Ok(Some(Misbehaviour {
                    client_id: update_client_event.client_id.to_string(),
                    header_1: Some(update_client_event.header.clone()),
                    header_2: Some(Header {
                        signed_header: Some(divergence.challenging_block.signed_header.into()),
                        validator_set: Some(divergence.challenging_block.validators.into()),
                        trusted_height: Some(latest_trusted_height),
                        trusted_validators: Some(latest_trusted_validator_set.into()),
                    }),
                }))
            }
            None => {
                chain
                    .log(
                        "No divergence found while checking for misbehaviour",
                        &LevelDebug,
                    )
                    .await;
                Ok(None)
            }
        }
    }
}
