use core::marker::PhantomData;

use cgp_core::ErrorRaiser;
use cgp_core::{DelegateComponent, HasErrorType};

pub struct DelegateErrorRaiser<Components>(pub PhantomData<Components>);

impl<Context, Error, Components, Delegate> ErrorRaiser<Context, Error>
    for DelegateErrorRaiser<Components>
where
    Context: HasErrorType,
    Components: DelegateComponent<Error, Delegate = Delegate>,
    Delegate: ErrorRaiser<Context, Error>,
{
    fn raise_error(e: Error) -> Context::Error {
        Delegate::raise_error(e)
    }
}
