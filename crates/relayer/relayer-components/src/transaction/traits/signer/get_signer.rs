use hermes_prelude::*;

use crate::transaction::traits::HasSignerType;

#[cgp_component {
    provider: SignerGetter,
    context: Chain,
}]
pub trait HasSigner: HasSignerType + HasAsyncErrorType {
    fn get_signer(&self, signer_index: usize) -> Result<&Self::Signer, Self::Error>;
}
