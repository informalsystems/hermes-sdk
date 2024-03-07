use cgp_core::prelude::*;
use hermes_relayer_components::encode::impls::convert::TryConvertFrom;
use hermes_relayer_components::encode::traits::convert::Converter;
use ibc_proto::ibc::lightclients::tendermint::v1::ClientState as ProtoTendermintClientState;
use ibc_relayer_types::clients::ics07_tendermint::client_state::ClientState as TendermintClientState;

pub struct CosmosConverterComponents;

delegate_components! {
    CosmosConverterComponents {
        (TendermintClientState, ProtoTendermintClientState): ConvertTendermintClientState,
        (ProtoTendermintClientState, TendermintClientState): TryConvertFrom,
    }
}

pub struct ConvertTendermintClientState;

impl<Encoding> Converter<Encoding, TendermintClientState, ProtoTendermintClientState>
    for ConvertTendermintClientState
where
    Encoding: HasErrorType,
{
    fn convert(
        _encoding: &Encoding,
        value: &TendermintClientState,
    ) -> Result<ProtoTendermintClientState, Encoding::Error> {
        #[allow(deprecated)]
        let proto_client_state = ProtoTendermintClientState {
            chain_id: value.chain_id.to_string(),
            trust_level: Some(value.trust_threshold.into()),
            trusting_period: Some(value.trusting_period.into()),
            unbonding_period: Some(value.unbonding_period.into()),
            max_clock_drift: Some(value.max_clock_drift.into()),
            frozen_height: value.frozen_height.map(|height| height.into()),
            latest_height: Some(value.latest_height.into()),
            proof_specs: value.proof_specs.clone().into(),
            upgrade_path: value.upgrade_path.clone(),
            allow_update_after_expiry: value.allow_update.after_expiry,
            allow_update_after_misbehaviour: value.allow_update.after_misbehaviour,
        };

        Ok(proto_client_state)
    }
}
