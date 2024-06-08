use std::iter;
use std::str::FromStr;
use std::time::Duration;

use cgp_core::HasErrorType;
use eyre::{eyre, Error as ReportError};
use hermes_cosmos_chain_components::traits::chain_handle::HasBlockingChainHandle;
use hermes_cosmos_chain_components::types::tendermint::TendermintClientState;
use hermes_relayer_components::chain::traits::payload_builders::update_client::UpdateClientPayloadBuilder;
use hermes_relayer_components::chain::traits::queries::chain_status::CanQueryChainStatusAtHeight;
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::update_client::HasUpdateClientPayloadType;
use hermes_sovereign_rollup_components::types::client_state::WrappedSovereignClientState;
use hermes_sovereign_rollup_components::types::height::RollupHeight;
use hermes_sovereign_rollup_components::types::status::SovereignRollupStatus;
use ibc::clients::tendermint::types::Header;
use ibc::core::client::types::Height as DataChainHeight;
use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer::client_state::AnyClientState;
use ibc_relayer_types::clients::ics07_tendermint::client_state::AllowUpdate;
use ibc_relayer_types::core::ics02_client::header::AnyHeader;
use ibc_relayer_types::core::ics02_client::height::Height;
use ibc_relayer_types::core::ics02_client::trust_threshold::TrustThreshold as RelayerTrustThreshold;
use ibc_relayer_types::core::ics23_commitment::specs::ProofSpecs;
use ibc_relayer_types::core::ics24_host::identifier::ChainId as RelayerChainId;
use sov_celestia_client::types::client_state::TendermintClientParams;

use crate::sovereign::traits::chain::data_chain::{HasDataChain, HasDataChainType};
use crate::sovereign::traits::chain::rollup::HasRollup;
use crate::sovereign::types::payloads::client::SovereignUpdateClientPayload;

/**
   Build an update client payload from a Sovereign rollup, to be used later
   for sending an update client message to a Cosmos counterparty chain.
*/
pub struct BuildSovereignUpdateClientPayload;

impl<Chain, Counterparty, DataChain> UpdateClientPayloadBuilder<Chain, Counterparty>
    for BuildSovereignUpdateClientPayload
where
    Chain: HasHeightType<Height = RollupHeight>
        + HasUpdateClientPayloadType<Counterparty, UpdateClientPayload = SovereignUpdateClientPayload>
        + HasClientStateType<Counterparty, ClientState = WrappedSovereignClientState>
        + HasDataChain
        + HasRollup
        + HasDataChainType<DataChain = DataChain>
        + HasErrorType<Error = ReportError>
        + CanQueryChainStatusAtHeight<ChainStatus = SovereignRollupStatus>,
    Chain::DataChain: HasErrorType + HasBlockingChainHandle,
{
    async fn build_update_client_payload(
        chain: &Chain,
        trusted_height: &RollupHeight,
        target_height: &RollupHeight,
        client_state: Chain::ClientState,
    ) -> Result<SovereignUpdateClientPayload, Chain::Error> {
        // FIXME: the latest rollup height with +1 workaround is causing the DA header
        // verification to fail, as the DA has not progressed to the expected height.
        tokio::time::sleep(Duration::from_secs(2)).await;

        let sovereign_params = &client_state.sovereign_client_state.sovereign_params;

        // DA height is higher than rollup height. This requires adding
        // the genesis Height to the trusted and target Heights
        let da_trusted_height = Height::new(
            sovereign_params.genesis_da_height.revision_number(),
            trusted_height.slot_number + sovereign_params.genesis_da_height.revision_height(),
        )
        .map_err(|e| eyre!("Error creating DA Height: {e}"))?;

        let da_target_height = Height::new(
            sovereign_params.genesis_da_height.revision_number(),
            target_height.slot_number + sovereign_params.genesis_da_height.revision_height(),
        )
        .map_err(|e| eyre!("Error creating DA Height: {e}"))?;

        let rollup_trusted_height = DataChainHeight::new(
            sovereign_params.latest_height.revision_number(),
            trusted_height.slot_number,
        )
        .map_err(|e| eyre!("Error creating Rollup trusted Height: {e}"))?;

        let rollup_target_height = DataChainHeight::new(
            sovereign_params.latest_height.revision_number(),
            target_height.slot_number,
        )
        .map_err(|e| eyre!("Error creating Rollup target Height: {e}"))?;

        let data_chain = chain.data_chain();

        let da_client_state = convert_tm_params_to_client_state(
            &client_state.sovereign_client_state.da_params,
            &da_target_height,
        )?;

        let headers = data_chain
            .with_blocking_chain_handle(move |chain_handle| {
                let (header, support) = chain_handle
                    .build_header(
                        da_trusted_height,
                        da_target_height,
                        AnyClientState::Tendermint(da_client_state),
                    )
                    .unwrap();

                let headers = iter::once(header)
                    .chain(support.into_iter())
                    .map(|header| match header {
                        AnyHeader::Tendermint(header) => {
                            let da_height = DataChainHeight::new(
                                header.trusted_height.revision_number(),
                                header.trusted_height.revision_height(),
                            )
                            .map_err(|e| eyre!("Error creating DA Height: {e}"))
                            .unwrap();
                            let da_header = Header {
                                signed_header: header.signed_header.clone(),
                                validator_set: header.validator_set.clone(),
                                trusted_height: da_height,
                                trusted_next_validator_set: header.trusted_validator_set.clone(),
                            };
                            Ok(da_header)
                        }
                    })
                    .collect::<Result<Vec<Header>, Chain::Error>>()
                    .unwrap();

                Ok(headers)
            })
            .await
            .map_err(|e| eyre!("Error creating headers from DA chain: {e:?}"))?;

        let chain_status = chain.query_chain_status_at_height(target_height).await?;

        assert_eq!(&chain_status.height, target_height);

        println!(
            "update client payload: rollup user hash at {}: {:?}",
            target_height, chain_status.user_hash
        );

        Ok(SovereignUpdateClientPayload {
            datachain_header: headers,
            initial_state_height: rollup_trusted_height,
            final_state_height: rollup_target_height,
            final_user_hash: chain_status.user_hash,
            final_kernel_hash: chain_status.kernel_hash,
            final_root_hash: chain_status.root_hash,
        })
    }
}

/// This is a temporary solution which converts the TendermintParams to Tendermint ClientState.
/// The Sovereign client state only has a TendermintParams field, but in order to build the
/// client update payload, the DA chain's client state is required.
/// Until the Light client is decoupled from the Cosmos SDK in order to build the DA header
/// half the Tendermint ClientState value are mocked.
/// See issue: https://github.com/informalsystems/hermes-sdk/issues/204
fn convert_tm_params_to_client_state(
    tm_params: &TendermintClientParams,
    da_target_height: &Height,
) -> Result<TendermintClientState, ReportError> {
    let relayer_chain_id = RelayerChainId::from_str(&tm_params.chain_id.to_string())
        .map_err(|e| eyre!("Error converting ChainId to Relayer Chain Id: {e}"))?;
    let relayer_trust_threshold = RelayerTrustThreshold::new(
        tm_params.trust_level.numerator(),
        tm_params.trust_level.denominator(),
    )
    .map_err(|e| eyre!("Error converting TrustThreshold to Relayer TrustThreshold: {e}"))?;
    Ok(TendermintClientState {
        chain_id: relayer_chain_id,
        trust_threshold: relayer_trust_threshold,
        // trusting_period was removed from `TendermintClientParams`
        // https://github.com/informalsystems/sovereign-ibc/commit/a9aaa80c4fe7b21fa777ae2a186838aac1fed68c#diff-8735596286f5213c6003fc9dc4c719fe9c9d4f14b7a385f1418f766ef48faa54L17
        trusting_period: Duration::from_secs(300),
        unbonding_period: tm_params.unbonding_period,
        max_clock_drift: tm_params.max_clock_drift,
        latest_height: *da_target_height,
        proof_specs: ProofSpecs::default(),
        upgrade_path: vec![],
        allow_update: AllowUpdate {
            after_expiry: false,
            after_misbehaviour: false,
        },
        frozen_height: None,
    })
}
