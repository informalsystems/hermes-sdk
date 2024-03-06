use cgp_core::prelude::*;
use hermes_protobuf_components::impls::protobuf::EncodeAsProtobuf;
use hermes_protobuf_components::impls::wrap_any::EncodeWrapAny;
use hermes_relayer_components::encode::impls::convert_and_encode::ConvertAndEncode;
use hermes_relayer_components::encode::traits::decoder::Decoder;
use hermes_relayer_components::encode::traits::encoded::HasEncodedType;
use hermes_relayer_components::encode::traits::encoder::Encoder;
use hermes_relayer_components::encode::types::wrap::Wrap;
use ibc_proto::ibc::lightclients::tendermint::v1::ClientState as ProtoTendermintClientState;
use ibc_relayer_types::clients::ics07_tendermint::client_state::ClientState as TendermintClientState;
use prost_types::Any;

pub struct CosmosEncoderComponents;

delegate_components! {
    CosmosEncoderComponents {
        Wrap<Any, TendermintClientState>: EncodeWrapAny,

        TendermintClientState: ConvertAndEncode<ProtoTendermintClientState>,

        ProtoTendermintClientState: EncodeAsProtobuf,
    }
}

impl<Encoding, Value, Delegate> Encoder<Encoding, Value> for CosmosEncoderComponents
where
    Encoding: HasEncodedType + HasErrorType,
    Self: DelegateComponent<Value, Delegate = Delegate>,
    Delegate: Encoder<Encoding, Value>,
{
    fn encode(encoding: &Encoding, value: &Value) -> Result<Encoding::Encoded, Encoding::Error> {
        Delegate::encode(encoding, value)
    }
}

impl<Encoding, Value, Delegate> Decoder<Encoding, Value> for CosmosEncoderComponents
where
    Encoding: HasEncodedType + HasErrorType,
    Self: DelegateComponent<Value, Delegate = Delegate>,
    Delegate: Decoder<Encoding, Value>,
{
    fn decode(encoding: &Encoding, encoded: &Encoding::Encoded) -> Result<Value, Encoding::Error> {
        Delegate::decode(encoding, encoded)
    }
}
