use cgp_core::{CanRaiseError, HasErrorType};
use hermes_protobuf_components::impls::any::TypeUrlMismatchError;

pub trait CanAssertTypeUrlMatches: HasErrorType {
    fn assert_type_url_matches(expected: &str, actual: &str) -> Result<(), Self::Error>;
}

impl<Context> CanAssertTypeUrlMatches for Context
where
    Context: CanRaiseError<TypeUrlMismatchError>,
{
    fn assert_type_url_matches(expected: &str, actual: &str) -> Result<(), Self::Error> {
        if expected == actual {
            Ok(())
        } else {
            Err(Context::raise_error(TypeUrlMismatchError {
                expected_url: expected.to_owned(),
                actual_url: actual.to_owned(),
            }))
        }
    }
}
