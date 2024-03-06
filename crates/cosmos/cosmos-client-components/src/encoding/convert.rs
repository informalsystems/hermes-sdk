use cgp_core::prelude::*;
use hermes_relayer_components::encode::impls::convert::{ConvertFrom, TryConvertFrom};
use hermes_relayer_components::encode::traits::convert::Converter;
use ibc_proto::ibc::lightclients::tendermint::v1::ClientState as ProtoTendermintClientState;
use ibc_relayer_types::clients::ics07_tendermint::client_state::ClientState as TendermintClientState;

pub struct CosmosConverterComponents;

impl<Encoding, To, From, Delegate> Converter<Encoding, From, To> for CosmosConverterComponents
where
    Encoding: HasErrorType,
    Self: DelegateComponent<(To, From), Delegate = Delegate>,
    Delegate: Converter<Encoding, From, To>,
{
    fn convert(encoding: &Encoding, from: &From) -> Result<To, Encoding::Error> {
        Delegate::convert(encoding, from)
    }
}

delegate_components! {
    CosmosConverterComponents {
        (TendermintClientState, ProtoTendermintClientState): TryConvertFrom,
        (ProtoTendermintClientState, TendermintClientState): ConvertFrom,
    }
}
