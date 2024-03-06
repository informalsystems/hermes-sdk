use cgp_core::delegate_all;
use cgp_core::prelude::*;
use cgp_core::ErrorRaiserComponent;
use cgp_core::ErrorTypeComponent;
use hermes_cosmos_client_components::encoding::components::{
    CosmosEncodingComponents as BaseCosmosEncodingComponents, IsCosmosEncodingComponent,
};
use hermes_protobuf_components::traits::encoding::HasProtobufEncodingType;
use hermes_protobuf_components::traits::encoding::ProtobufEncodingGetter;
use hermes_protobuf_components::traits::encoding::ProvideProtobufEncodingType;
use hermes_relayer_components::encode::traits::decoder::CanDecode;
use hermes_relayer_components::encode::traits::encoded::HasEncodedType;
use hermes_relayer_components::encode::traits::encoder::CanEncode;
use hermes_relayer_components::encode::types::via::Via;
use ibc_proto::ibc::lightclients::tendermint::v1::ClientState as ProtoTendermintClientState;
use ibc_relayer_types::clients::ics07_tendermint::client_state::ClientState as TendermintClientState;
use prost_types::Any;

use crate::impls::error::HandleCosmosError;

#[derive(Default)]
pub struct CosmosEncoding;

pub struct CosmosEncodingComponents;

impl HasComponents for CosmosEncoding {
    type Components = CosmosEncodingComponents;
}

delegate_all!(
    IsCosmosEncodingComponent,
    BaseCosmosEncodingComponents,
    CosmosEncodingComponents,
);

delegate_components! {
    CosmosEncodingComponents {
        [
            ErrorTypeComponent,
            ErrorRaiserComponent,
        ]:
            HandleCosmosError,
    }
}

pub struct ProvideCosmosEncoding;

impl<Context> ProvideProtobufEncodingType<Context> for ProvideCosmosEncoding
where
    Context: Async,
{
    type Encoding = CosmosEncoding;
}

impl<Context> ProtobufEncodingGetter<Context> for ProvideCosmosEncoding
where
    Context: HasProtobufEncodingType<Encoding = CosmosEncoding>,
{
    fn encoding(_context: &Context) -> &CosmosEncoding {
        &CosmosEncoding
    }
}

pub trait CheckCosmosEncoding:
    HasEncodedType<Encoded = Vec<u8>>
    + CanEncode<ProtoTendermintClientState>
    + CanDecode<ProtoTendermintClientState>
    + CanEncode<TendermintClientState>
    + CanDecode<TendermintClientState>
    + CanEncode<Via<Any, TendermintClientState>>
    + CanDecode<Via<Any, TendermintClientState>>
{
}

impl CheckCosmosEncoding for CosmosEncoding {}
