use cgp_core::HasErrorType;
use eyre::Error;

use crate::bootstrap::types::bootstrap::CosmosBootstrapContext;

impl HasErrorType for CosmosBootstrapContext {
    type Error = Error;
}
