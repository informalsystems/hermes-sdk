use core::marker::PhantomData;

use cgp::prelude::HasField;

pub trait FieldGetter<Context> {
    type Field;

    fn get_field(context: &Context) -> &Self::Field;
}

pub struct GetField<Tag>(pub PhantomData<Tag>);

impl<Context, Tag> FieldGetter<Context> for GetField<Tag>
where
    Context: HasField<Tag>,
{
    type Field = Context::Field;

    fn get_field(context: &Context) -> &Context::Field {
        context.get_field(PhantomData)
    }
}
