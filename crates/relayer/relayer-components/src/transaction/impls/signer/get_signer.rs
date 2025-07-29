use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::marker::PhantomData;

use hermes_prelude::*;

use crate::transaction::traits::{HasSignerType, SignerGetter, SignerGetterComponent};

pub struct SignerWithIndexGetter;

#[cgp_provider(SignerGetterComponent)]
impl<Chain> SignerGetter<Chain> for SignerWithIndexGetter
where
    Chain: HasSignerType
        + HasField<symbol!("key_entry"), Value = Chain::Signer>
        + HasField<symbol!("additional_key_entries"), Value = Vec<Chain::Signer>>
        + CanRaiseAsyncError<String>,
{
    fn get_signer(chain: &Chain, signer_index: usize) -> Result<&Chain::Signer, Chain::Error> {
        if signer_index == 0 {
            Ok(chain.get_field(PhantomData::<symbol!("key_entry")>))
        } else {
            // Since we take the `key_entry` with index 0, the index for additional keys
            // needs to be 1 less than the actual index
            let updated_index = signer_index - 1;
            let additional_signers =
                chain.get_field(PhantomData::<symbol!("additional_key_entries")>);
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
