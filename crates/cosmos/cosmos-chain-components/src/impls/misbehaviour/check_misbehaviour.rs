use core::marker::PhantomData;

use hermes_comet_light_client_components::traits::{CanDetectMisbehaviour, CanFetchLightBlock};
use hermes_comet_light_client_context::contexts::light_client::CometLightClient;
use hermes_core::chain_components::traits::{
    CanExtractFromEvent, CanQueryChainHeight, CanQueryClientState, HasChainId, HasClientStateType,
    HasEvidenceType, HasUpdateClientEvent, MisbehaviourChecker, MisbehaviourCheckerComponent,
};
use hermes_core::logging_components::traits::CanLog;
use hermes_core::logging_components::types::{LevelDebug, LevelError, LevelWarn};
use hermes_error::HermesError;
use hermes_prelude::*;
use ibc::core::client::types::Height;
use ibc::core::host::types::identifiers::ClientId;
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
    Chain: HasEvidenceType<Evidence = Misbehaviour>
        + HasUpdateClientEvent
        + HasRpcClient
        + HasChainId
        + CanQueryClientState<Counterparty, ClientId = ClientId>
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
    Counterparty: HasClientStateType<Chain>,
    TendermintClientState: From<Counterparty::ClientState>,
{
    async fn check_misbehaviour(
        chain: &Chain,
        update_client_event: &Chain::UpdateClientEvent,
    ) -> Result<Option<Chain::Evidence>, Chain::Error> {
        let raw_trusted_height = update_client_event.header.trusted_height.unwrap();
        let trusted_height = Height::try_from(raw_trusted_height).unwrap();
        let tm_trusted_height = TendermintHeight::try_from(trusted_height.revision_height())
            .map_err(Chain::raise_error)?;
        let client_state = chain
            .query_client_state(PhantomData, &update_client_event.client_id, &trusted_height)
            .await?;

        let tm_client_state = TendermintClientState::from(client_state);

        let update_header = update_client_event.header.clone();

        let latest_height = chain.query_chain_height().await?;

        let rpc_client = chain.rpc_client().clone();

        let tendermint_latest_height = TendermintHeight::try_from(latest_height.revision_height())
            .map_err(Chain::raise_error)?;

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

        let maybe_divergence = light_client
            .detect(&target_block, &trusted_block)
            .await
            .map_err(Chain::raise_error)?;

        match maybe_divergence {
            Some(divergence) => {
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
                    current_trusted_height = header.trusted_height.unwrap().try_into().unwrap();

                    let next_height = TendermintHeight::try_from(
                        header.trusted_height.unwrap().revision_height + 1,
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
                                prev_header.trusted_height.unwrap().revision_height + 1,
                            )
                            .map_err(Chain::raise_error)?;
                            let prev_succ = light_client
                                .fetch_light_block(&prev_height)
                                .await
                                .map_err(Chain::raise_error)?;
                            (prev_header.trusted_height.unwrap(), prev_succ.validators)
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
            None => Ok(None),
        }
    }
}
