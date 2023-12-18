use cgp_core::{CanRaiseError, HasErrorType};

pub trait HasRuntime: HasErrorType + CanRaiseError<<Self::Runtime as HasErrorType>::Error> {
    type Runtime: HasErrorType;

    fn runtime(&self) -> &Self::Runtime;
}
