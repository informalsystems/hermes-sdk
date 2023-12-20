use cgp_core::Async;
use hermes_relayer_components::runtime::traits::runtime::ProvideRuntimeType;

use crate::types::runtime::TokioRuntimeContext;

pub struct ProvideTokioRuntimeType;

impl<Context> ProvideRuntimeType<Context> for ProvideTokioRuntimeType
where
    Context: Async,
{
    type Runtime = TokioRuntimeContext;
}
