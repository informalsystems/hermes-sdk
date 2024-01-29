use cgp_core::HasErrorType;

use crate::runtime::traits::mutex::HasMutex;
use crate::runtime::traits::runtime::HasRuntimeType;

pub type RuntimeOf<Context> = <Context as HasRuntimeType>::Runtime;

pub type MutexOf<Context, T> = <RuntimeOf<Context> as HasMutex>::Mutex<T>;

pub type ErrorOf<Context> = <Context as HasErrorType>::Error;
