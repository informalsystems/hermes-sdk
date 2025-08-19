use core::marker::PhantomData;

use hermes_chain_type_components::traits::HasChainIdType;
use hermes_prelude::*;

use crate::batch::types::config::BatchConfig;

pub trait HasBatchConfig<I, Chain>: HasAsyncErrorType
where
    Chain: HasChainIdType,
{
    fn batch_config(
        &self,
        _tag: PhantomData<I>,
        chain_id: &Chain::ChainId,
    ) -> Result<BatchConfig, Self::Error>;
}
