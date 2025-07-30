use core::marker::PhantomData;

use hermes_core::chain_components::traits::HasChainId;
use hermes_core::relayer_components::transaction::traits::{
    DefaultSignerGetter, DefaultSignerGetterComponent, HasSignerType,
};
use hermes_prelude::*;

#[cgp_new_provider(DefaultSignerGetterComponent)]
impl<Chain, Tag> DefaultSignerGetter<Chain> for GetFirstSignerAsDefault<Tag>
where
    Chain: HasSignerType + HasChainId + HasField<Tag, Value = Vec<Chain::Signer>>,
{
    fn get_default_signer(chain: &Chain) -> &Chain::Signer {
        let signers = chain.get_field(PhantomData);
        signers
            .first()
            .unwrap_or_else(|| panic!("chain `{}` doesn't have any signers", chain.chain_id()))
    }
}
