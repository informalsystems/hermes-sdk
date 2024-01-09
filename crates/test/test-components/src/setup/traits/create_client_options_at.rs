use hermes_relayer_components::chain::traits::types::create_client::HasCreateClientOptionsType;

use crate::driver::traits::types::chain_at::{ChainTypeAt, HasChainTypeAt};
use crate::setup::traits::driver::HasDriverType;

pub trait HasCreateClientOptionsAt<const TARGET: usize, const COUNTERPARTY: usize>:
    HasDriverType
where
    Self::Driver: HasChainTypeAt<TARGET> + HasChainTypeAt<COUNTERPARTY>,
    ChainTypeAt<Self::Driver, TARGET>:
        HasCreateClientOptionsType<ChainTypeAt<Self::Driver, COUNTERPARTY>>,
{
    fn create_client_options(
        &self,
    ) -> &<ChainTypeAt<Self::Driver, TARGET> as HasCreateClientOptionsType<
        ChainTypeAt<Self::Driver, COUNTERPARTY>,
    >>::CreateClientPayloadOptions;
}
