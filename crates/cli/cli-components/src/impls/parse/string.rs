use core::marker::PhantomData;
use core::str::FromStr;

use hermes_prelude::*;

use crate::traits::{ArgParser, ArgParserComponent};

pub struct ParseFromString<Parsed>(pub PhantomData<Parsed>);

#[cgp_provider(ArgParserComponent)]
impl<App, Args, Tag, Parsed> ArgParser<App, Args, Tag> for ParseFromString<Parsed>
where
    App: CanRaiseAsyncError<Parsed::Err>,
    Args: HasField<Tag, Value = String>,
    Parsed: Async + FromStr,
{
    type Parsed = Parsed;

    fn parse_arg(_app: &App, args: &Args, _tag: PhantomData<Tag>) -> Result<Parsed, App::Error> {
        args.get_field(PhantomData)
            .parse()
            .map_err(App::raise_error)
    }
}

pub struct ParseFromOptionalString<Parsed>(pub PhantomData<Parsed>);

#[cgp_provider(ArgParserComponent)]
impl<App, Args, Tag, Parsed> ArgParser<App, Args, Tag> for ParseFromOptionalString<Parsed>
where
    App: CanRaiseAsyncError<Parsed::Err>,
    Args: HasField<Tag, Value = Option<String>>,
    Parsed: Async + FromStr,
{
    type Parsed = Option<Parsed>;

    fn parse_arg(
        _app: &App,
        args: &Args,
        _tag: PhantomData<Tag>,
    ) -> Result<Option<Parsed>, App::Error> {
        let m_field = args.get_field(PhantomData);

        match m_field {
            Some(field) => {
                let parsed = field.parse().map_err(App::raise_error)?;

                Ok(Some(parsed))
            }
            None => Ok(None),
        }
    }
}
