use core::marker::PhantomData;

use cgp_core::component::DelegateComponent;
use cgp_core::error::HasErrorType;

use crate::traits::parse::ArgParser;

pub struct DelegateArgParsers<Components>(pub PhantomData<Components>);

impl<App, Args, Tag, Components, Delegate> ArgParser<App, Args, Tag>
    for DelegateArgParsers<Components>
where
    App: HasErrorType,
    Components: DelegateComponent<(Args, Tag), Delegate = Delegate>,
    Delegate: ArgParser<App, Args, Tag>,
{
    type Parsed = Delegate::Parsed;

    fn parse_arg(
        app: &App,
        args: &Args,
        tag: PhantomData<Tag>,
    ) -> Result<Self::Parsed, App::Error> {
        Delegate::parse_arg(app, args, tag)
    }
}
