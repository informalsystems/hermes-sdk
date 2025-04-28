use hermes_core::encoding_components::impls::DecodeFrom;
use hermes_core::encoding_components::traits::{
    HasEncodeBufferType, MutDecoderComponent, MutEncoder, MutEncoderComponent, Transformer,
};
use hermes_prelude::*;
use hermes_protobuf_encoding_components::impls::EncodeByteField;
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

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, CommitmentRoot> for EncodeCommitmentRoot
where
    Encoding: HasEncodeBufferType + HasAsyncErrorType,
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
