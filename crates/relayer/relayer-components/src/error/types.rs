use cgp_core::HasErrorType;

pub type ErrorOf<Context> = <Context as HasErrorType>::Error;
