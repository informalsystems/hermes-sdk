use cgp_core::traits::HasErrorType;

use crate::one_for_all::traits::builder::OfaBuilder;
use crate::one_for_all::types::builder::OfaBuilderWrapper;

impl<Builder> HasErrorType for OfaBuilderWrapper<Builder>
where
    Builder: OfaBuilder,
{
    type Error = Builder::Error;
}
