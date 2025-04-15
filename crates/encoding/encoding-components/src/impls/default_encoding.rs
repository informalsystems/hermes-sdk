use cgp::prelude::*;

use crate::traits::{EncodingGetter, EncodingGetterComponent, HasDefaultEncoding};

pub struct GetDefaultEncoding;

#[cgp_provider(EncodingGetterComponent<Kind>)]
impl<Context, Kind> EncodingGetter<Context, Kind> for GetDefaultEncoding
where
    Context: HasDefaultEncoding<Kind>,
{
    fn encoding(_context: &Context) -> &Context::Encoding {
        Context::default_encoding()
    }
}
