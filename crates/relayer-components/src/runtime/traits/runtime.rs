use cgp_core::HasErrorType;

pub trait HasRuntime: HasErrorType {
    type Runtime: HasErrorType;

    fn runtime(&self) -> &Self::Runtime;

    fn runtime_error(e: <Self::Runtime as HasErrorType>::Error) -> Self::Error;
}
