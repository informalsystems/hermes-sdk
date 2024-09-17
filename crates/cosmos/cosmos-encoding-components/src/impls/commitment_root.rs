use cgp::prelude::*;
use hermes_encoding_components::impls::encode_mut::from::DecodeFrom;
use hermes_encoding_components::traits::encode_mut::MutEncoder;
use hermes_encoding_components::traits::transform::Transformer;
use hermes_encoding_components::traits::types::encode_buffer::HasEncodeBufferType;
use hermes_protobuf_encoding_components::components::MutDecoderComponent;
use hermes_protobuf_encoding_components::impls::encode_mut::proto_field::bytes::EncodeByteField;
use ibc::core::commitment_types::commitment::CommitmentRoot;

pub struct EncodeCommitmentRoot;

delegate_components! {
    EncodeCommitmentRoot {
        MutDecoderComponent: DecodeFrom<
            Self,
            EncodeByteField<1>
        >,
    }
}

impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, CommitmentRoot> for EncodeCommitmentRoot
where
    Encoding: HasEncodeBufferType + HasErrorType,
    EncodeByteField<1>: for<'a> MutEncoder<Encoding, Strategy, &'a [u8]>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &CommitmentRoot,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        EncodeByteField::encode_mut(encoding, &value.as_bytes(), buffer)?;

        Ok(())
    }
}

impl Transformer for EncodeCommitmentRoot {
    type From = Vec<u8>;

    type To = CommitmentRoot;

    fn transform(from: Vec<u8>) -> CommitmentRoot {
        CommitmentRoot::from(from)
    }
}
