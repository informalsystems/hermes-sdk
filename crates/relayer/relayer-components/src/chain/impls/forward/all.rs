use cgp_core::prelude::*;

use crate::chain::impls::forward::chain_id::ForwardChainId;
use crate::chain::impls::forward::create_client::ForwardCreateClientOptionsType;
use crate::chain::impls::forward::init_channel::ForwardInitChannelOptionsType;
use crate::chain::impls::forward::init_connection::ForwardInitConnectionOptionsType;
use crate::chain::traits::types::chain_id::{ChainIdGetterComponent, ChainIdTypeProviderComponent};
use crate::chain::traits::types::channel::InitChannelOptionsTypeComponent;
use crate::chain::traits::types::connection::InitConnectionOptionsTypeComponent;
use crate::chain::traits::types::create_client::CreateClientOptionsTypeComponent;

pub struct ForwardToInnerChain;

delegate_components! {
    ForwardToInnerChain {
        [
            ChainIdTypeProviderComponent,
            ChainIdGetterComponent,
        ]:
            ForwardChainId,
        CreateClientOptionsTypeComponent:
            ForwardCreateClientOptionsType,
        InitConnectionOptionsTypeComponent:
            ForwardInitConnectionOptionsType,
        InitChannelOptionsTypeComponent:
            ForwardInitChannelOptionsType,
    }
}
