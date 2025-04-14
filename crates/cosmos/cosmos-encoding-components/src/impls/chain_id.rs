use cgp::prelude::*;
use hermes_encoding_components::traits::{
    HasDecodeBufferType, HasEncodeBufferType, MutDecoder, MutDecoderComponent, MutEncoder,
    MutEncoderComponent,
};
use hermes_protobuf_encoding_components::impls::encode_mut::proto_field::string::EncodeStringField;
use ibc::core::host::types::error::IdentifierError;
use ibc::core::host::types::identifiers::ChainId;

pub struct EncodeChainIdField<const TAG: u32>;

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy, const TAG: u32> MutEncoder<Encoding, Strategy, ChainId>
    for EncodeChainIdField<TAG>
where
    Encoding: HasEncodeBufferType + HasAsyncErrorType,
    EncodeStringField<TAG>: MutEncoder<Encoding, Strategy, String>,
{
    fn encode_mut(
        encoding: &Encoding,
        chain_id: &ChainId,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        <EncodeStringField<TAG>>::encode_mut(encoding, &chain_id.to_string(), buffer)?;

        Ok(())
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy, const TAG: u32> MutDecoder<Encoding, Strategy, ChainId>
    for EncodeChainIdField<TAG>
where
    Encoding: HasDecodeBufferType + CanRaiseAsyncError<IdentifierError>,
    EncodeStringField<TAG>: MutDecoder<Encoding, Strategy, String>,
{
    fn decode_mut<'a>(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'_>,
    ) -> Result<ChainId, Encoding::Error> {
        let chain_id_str = <EncodeStringField<TAG>>::decode_mut(encoding, buffer)?;

        ChainId::new(&chain_id_str).map_err(Encoding::raise_error)
    }
}
