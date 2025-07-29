use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::marker::PhantomData;

use hermes_prelude::*;

use crate::transaction::traits::{HasSignerType, SignerGetter, SignerGetterComponent};

#[cgp_new_provider(SignerGetterComponent)]
impl<Chain, Tag, AdditionalTag> SignerGetter<Chain> for SignerWithIndexGetter<Tag, AdditionalTag>
where
    Chain: HasSignerType
        + HasField<Tag, Value = Chain::Signer>
        + HasField<AdditionalTag, Value = Vec<Chain::Signer>>
        + CanRaiseAsyncError<String>,
{
    fn get_signer(chain: &Chain, signer_index: usize) -> Result<&Chain::Signer, Chain::Error> {
        if signer_index == 0 {
            Ok(chain.get_field(PhantomData::<Tag>))
        } else {
            // Since we take the `key_entry` with index 0, the index for additional keys
            // needs to be 1 less than the actual index
            let updated_index = signer_index - 1;
            let additional_signers = chain.get_field(PhantomData::<AdditionalTag>);
            if additional_signers.len() <= updated_index {
                return Err(Chain::raise_error(format!(
                    "index {updated_index} || {signer_index} is out of range to retrieve signer"
                )));
            }
            let signer = additional_signers.get(updated_index).ok_or_else(|| {
                Chain::raise_error(format!(
                    "index {updated_index} is out of range to retrieve signer"
                ))
            })?;
            Ok(signer)
        }
    }
}
