use cgp_core::prelude::*;
use cgp_core::{ErrorRaiserComponent, ErrorTypeComponent};
use hermes_cosmos_chain_components::encoding::components::*;
use hermes_cosmos_chain_components::types::tendermint::TendermintConsensusState;
use hermes_encoding_components::impls::default_encoding::GetDefaultEncoding;
use hermes_encoding_components::traits::convert::CanConvertBothWays;
use hermes_encoding_components::traits::encode_and_decode::CanEncodeAndDecode;
use hermes_encoding_components::traits::encoded::HasEncodedType;
use hermes_encoding_components::traits::has_encoding::{
    DefaultEncodingGetter, EncodingGetterComponent, HasEncodingType, ProvideEncodingType,
};
use hermes_protobuf_encoding_components::types::Protobuf;
use ibc::core::commitment_types::merkle::MerkleProof;
use ibc_relayer_types::clients::ics07_tendermint::client_state::ClientState as TendermintClientState;
use prost_types::Any;

use crate::impls::error::HandleCosmosError;

pub struct CosmosEncoding;

pub struct CosmosEncodingComponents2;

impl HasComponents for CosmosEncoding {
    type Components = CosmosEncodingComponents2;
}

with_cosmos_encoding_components! {
    delegate_components! {
        CosmosEncodingComponents2 {
            @CosmosEncodingComponents: CosmosEncodingComponents,
        }
    }
}

delegate_components! {
    CosmosEncodingComponents2 {
        [
            ErrorTypeComponent,
            ErrorRaiserComponent,
        ]:
            HandleCosmosError,
    }
}

pub struct ProvideCosmosEncoding;

delegate_components! {
    ProvideCosmosEncoding {
        EncodingGetterComponent: GetDefaultEncoding,
    }
}

impl<Context> ProvideEncodingType<Context> for ProvideCosmosEncoding
where
    Context: Async,
{
    type Encoding = CosmosEncoding;
}

impl<Context> DefaultEncodingGetter<Context> for ProvideCosmosEncoding
where
    Context: HasEncodingType<Encoding = CosmosEncoding>,
{
    fn default_encoding() -> &'static CosmosEncoding {
        &CosmosEncoding
    }
}

pub trait CheckCosmosEncoding:
    HasEncodedType<Encoded = Vec<u8>>
    + CanEncodeAndDecode<Protobuf, Vec<u8>>
    + CanEncodeAndDecode<Protobuf, TendermintClientState>
    + CanEncodeAndDecode<Protobuf, TendermintConsensusState>
    + CanEncodeAndDecode<Protobuf, MerkleProof>
    + CanEncodeAndDecode<Any, TendermintClientState>
    + CanEncodeAndDecode<Any, TendermintConsensusState>
    + CanConvertBothWays<Any, TendermintClientState>
    + CanConvertBothWays<Any, TendermintConsensusState>
{
}

impl CheckCosmosEncoding for CosmosEncoding {}
