use crate::traits::has_encoding::{EncodingGetter, HasDefaultEncoding};

pub struct GetDefaultEncoding;

impl<Context, Kind> EncodingGetter<Context, Kind> for GetDefaultEncoding
where
    Context: HasDefaultEncoding<Kind>,
{
    fn encoding(_context: &Context) -> &Context::Encoding {
        Context::default_encoding()
    }
}
