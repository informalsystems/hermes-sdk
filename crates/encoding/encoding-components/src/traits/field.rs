pub trait OwnedFieldGetter<Context> {
    type Field;

    fn get_field(context: &Context) -> Self::Field;
}
