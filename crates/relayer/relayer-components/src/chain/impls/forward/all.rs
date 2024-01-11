use cgp_core::prelude::*;

use crate::chain::impls::forward::chain_id::ForwardChainId;
use crate::chain::traits::types::chain_id::{ChainIdGetterComponent, ChainIdTypeProviderComponent};

pub struct ForwardToInnerChain;

delegate_components! {
    ForwardToInnerChain {
        [
            ChainIdTypeProviderComponent,
            ChainIdGetterComponent,
        ]:
            ForwardChainId,
    }
}
