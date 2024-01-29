use cgp_core::HasErrorType;

use crate::runtime::traits::runtime::HasRuntimeType;

pub type RuntimeOf<Context> = <Context as HasRuntimeType>::Runtime;

pub type ErrorOf<Context> = <Context as HasErrorType>::Error;
