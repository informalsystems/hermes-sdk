use cgp_core::prelude::*;

use crate::bootstrap::types::bootstrap::CosmosBootstrapContext;

pub struct CosmosBootstrapComponents;

impl HasComponents for CosmosBootstrapContext {
    type Components = CosmosBootstrapComponents;
}

delegate_components!(
    CosmosBootstrapComponents;
);
