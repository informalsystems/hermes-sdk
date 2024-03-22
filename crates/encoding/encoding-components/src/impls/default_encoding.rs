use crate::traits::has_encoding::{EncodingGetter, HasDefaultEncoding};

pub struct GetDefaultEncoding;

impl<Context> EncodingGetter<Context> for GetDefaultEncoding
where
    Context: HasDefaultEncoding,
{
    fn encoding(_context: &Context) -> &Context::Encoding {
        Context::default_encoding()
    }
}
