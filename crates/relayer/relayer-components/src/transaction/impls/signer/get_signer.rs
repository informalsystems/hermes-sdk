use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::marker::PhantomData;

use hermes_prelude::*;

use crate::transaction::traits::{HasSignerType, SignerGetter, SignerGetterComponent};

#[cgp_new_provider(SignerGetterComponent)]
impl<Chain, Tag> SignerGetter<Chain> for SignerWithIndexGetter<Tag>
where
    Chain: HasSignerType + HasField<Tag, Value = Vec<Chain::Signer>> + CanRaiseAsyncError<String>,
{
    fn get_signer(chain: &Chain, signer_index: usize) -> Result<&Chain::Signer, Chain::Error> {
        let signers = chain.get_field(PhantomData::<Tag>);
        if signers.len() <= signer_index {
            return Err(Chain::raise_error(format!(
                "index {signer_index} || {signer_index} is out of range to retrieve signer"
            )));
        }
        let signer = signers.get(signer_index).ok_or_else(|| {
            Chain::raise_error(format!(
                "index {signer_index} is out of range to retrieve signer"
            ))
        })?;
        Ok(signer)
    }
}
