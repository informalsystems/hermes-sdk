use cgp_core::prelude::*;
use hermes_relayer_components::encode::impls::convert::{ConvertFrom, TryConvertFrom};
use ibc_proto::ibc::lightclients::tendermint::v1::ClientState as ProtoTendermintClientState;
use ibc_relayer_types::clients::ics07_tendermint::client_state::ClientState as TendermintClientState;

pub struct CosmosConverterComponents;

delegate_components! {
    CosmosConverterComponents {
        (TendermintClientState, ProtoTendermintClientState): TryConvertFrom,
        (ProtoTendermintClientState, TendermintClientState): ConvertFrom,
    }
}