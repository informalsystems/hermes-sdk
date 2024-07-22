use core::marker::PhantomData;
use core::str::FromStr;

use cgp_core::prelude::*;

use crate::traits::parse::ArgParser;

pub struct ParseFromString<Parsed>(pub PhantomData<Parsed>);

impl<App, Args, Tag, Parsed> ArgParser<App, Args, Tag> for ParseFromString<Parsed>
where
    App: CanRaiseError<Parsed::Err>,
    Args: HasField<Tag, Field = String>,
    Parsed: Async + FromStr,
{
    type Parsed = Parsed;

    fn parse_arg(_app: &App, args: &Args, _tag: PhantomData<Tag>) -> Result<Parsed, App::Error> {
        args.get_field(PhantomData)
            .parse()
            .map_err(App::raise_error)
    }
}
