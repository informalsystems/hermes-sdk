use core::str::FromStr;

use hermes_encoding_components::traits::{
    HasDecodeBufferType, HasEncodeBufferType, MutDecoder, MutDecoderComponent, MutEncoder,
    MutEncoderComponent,
};
use hermes_prelude::*;
use hermes_protobuf_encoding_components::impls::EncodeStringField;
use ibc::core::host::types::error::IdentifierError;
use ibc::core::host::types::identifiers::ClientId;

pub struct EncodeClientIdField<const TAG: u32>;

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy, const TAG: u32> MutEncoder<Encoding, Strategy, ClientId>
    for EncodeClientIdField<TAG>
where
    Encoding: HasEncodeBufferType + HasAsyncErrorType,
    EncodeStringField<TAG>: MutEncoder<Encoding, Strategy, String>,
{
    fn encode_mut(
        encoding: &Encoding,
        client_id: &ClientId,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        <EncodeStringField<TAG>>::encode_mut(encoding, &client_id.to_string(), buffer)?;

        Ok(())
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy, const TAG: u32> MutDecoder<Encoding, Strategy, ClientId>
    for EncodeClientIdField<TAG>
where
    Encoding: HasDecodeBufferType + CanRaiseAsyncError<IdentifierError>,
    EncodeStringField<TAG>: MutDecoder<Encoding, Strategy, String>,
{
    fn decode_mut<'a>(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'_>,
    ) -> Result<ClientId, Encoding::Error> {
        let client_id_str = <EncodeStringField<TAG>>::decode_mut(encoding, buffer)?;

        ClientId::from_str(&client_id_str).map_err(Encoding::raise_error)
    }
}
