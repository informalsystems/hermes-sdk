# Field Types

CGP makes extensive use of field types to implement access to fields within a struct or enum, without requiring the data type to implement custom traits. This pattern is also known as [datatype-generic programming](https://www.cs.ox.ac.uk/projects/utgp/school/andres.pdf), since we are writing code that is generic over a data type, yet still understand some of its structure.

## `symbol!` Macro

To reason about fields in a struct or enum, we first need to somehow represent the _name_ of each field as type. The `symbol!` macro provides a way for us to take a string literal, which is normally a _value_ of type `String` or `&str`, and turn it into a _type`.

The way `symbol!` works is by making use of the _const generics_ feature in Rust. We can see how const generics allows lifting of values into type be defining a simple `Char` type, which lifts a single `char`:

```rust
pub struct Char<const CHAR: char>;
```

With the above definition, we would get distinct types for every `char`, e.g. `Char<'a'>` is a different type from `Char<'b'>`.

Although const generics makes it possible to lift a single character, the issue is that complex types such as `&str` or `String` are not yet supported by const generics. To workaround that, we need some additional machinery to _combine_ multiple characters into a type-level string. We can do that by adding a second parameter to `Char`:

```rust
pub struct Char<const CHAR: char, Tail>(pub PhantomData<Tail>);
```

With the definition, we can now chain together multiple characters in the form `Char<'a', Char<'b', Char<'c'>, ...>>`. With this, we also need a "terminating character", which would be defined as `Nil`:

```
pub struct Nil;
```

The type `Nil` is equivalent to `()`, just that we use it to disambiguate its usage as a terminator type. With that, a type like `Char<'a', Char<'b', Char<'c'>, Nil>>` can now be used to represent the string `"abc"` as type.

Now that we know how to combine multiple characters into a single type, we can use the `symbol!` macro to perform the desugaring for us. So the same type above can be expressed simply as `symbol("abc")`.

## Greek Symbols

An unfortunate side effect of using a type like `Char` to represent type-level strings is that the error messages can be overly verbose. It can be challenging to glance on error messages that contain types like `Char<'a', Char<'b', Char<'c'>, Nil>>`, and understand quickly that it corresponds to the string `"abc"`.

Unfortunately, Rust do not provide easy way for us to customize how types are displayed, so we cannot provide a very clean formating of the type. But what we can do is to _shorten_ the representation of types like `Char` and `Nil`, so that they take less space to show up in error messages.

Ideally, these types should use at most one character for maximal space saving. But avoid clashing with other one-character identifiers, we opt to use _greek alphabets_ to represent the field types. So we can now redefine the `Char` type as follows:

```rust
pub struct ι<const CHAR: char, Tail>(pub PhantomData<Tail>);

pub use ι as Char;
```

We choose the Greek character `ι` (Iota) to replace `Char`, as it has the least visual cluttering as compared to other characters. We also re-export `ι` as `Char`, so that we can still use the ASCII representation when writing code.

Similarly, the `Nil` type is now renamed to ε:

```rust
pub struct ε;

pub use ε as Nil;
```

With the Greek alphabets, the error messages for type-level string will show `"abc"` as `ι<'a', ι<'b', ι<'c'>, ε>>`. Although it is still not as compact as the literal `"abc"`, it is at least shorter than the original type `Char<'a', Char<'b', Char<'c'>, Nil>>`. This would become more apparent for longer strings that contain more characters.

## `HasField` Trait

## Product Types

When writing code that access all fields in a struct, we typically would require access to the concrete struct so that we can walk through every field by their names. But when we do not have access to the concrete struct, we need to first turn it into a generic collection of values.

With CGP, we use the `Cons` and the `Nil` type to construct _type-level lists_, so that we can store arbitrary number of values from a struct into nested tuples. The `Cons` type is defined as follows:

```rust
pub struct π<Head, Tail>(pub Head, pub Tail);

pub use π as Cons;
```

Similar to `Char`, the `Cons` type contains a `Tail` that represent the rest of the list. But compared to `Char`, `Cons` allows types other than `char` to be the "content" of the current cell. To shorten its representation, we also use the Greek alphabet `π` to represent `Cons`.

Using `Cons` and `Nil`, we can now define a generic representation of any struct. For example, given the following struct:

```rust
pub struct Person {
    pub name: String,
    pub age: u8,
    pub address: String,
}
```

We can convert the struct into `Cons<String, Cons<u8, Cons<String, Nil>>>`, and retain all information in the struct. Using the Greek alphabets, the same type would be shown instead as `π<String, π<u8, π<String, ε>>>`.

It is worth noting that we could technically also use the native Rust tuples to represent the generic fields, such as `(String, (u8, (String, ())))`, or just `(String, u8, String)`. However, we choose to not use the convention, as it can be confusing for readers to distinguish whether a use of tuple is meant to be an ordinary Rust type, or a CGP product type.

Product types can be useful in implementing generic providers, such as for encoding and decoding. At a high level, the algorithm for such providers would be to first perform some operation on the head, and then recursively perform the same operation for every remaining element in the tail. The main difference is that we are operating on a _heterogeneous_ list with each of the element having different type.

## Field Types

When converting the `Person` struct to a product type earlier, we do lose some information about the _name_ of the original fields. For instance, given that both the `name` and `address` fields have the type `String`, it is not clear whether the first `String` element in the product type refers to the `name` field or the `address` field.

To help disambiguate the fields, we also introduce a `Field` type that carries the field names in addition to the field values.

```rust
pub struct ω<Tag, Value> {
    pub value: Value,
    pub phantom: PhantomData<Tag>,
}

pub use ω as Field;
```

The `Field` type is parameterized by a `Tag` and a `Value`, but the `Tag` is phantom and it technically just wraps around `Value`. Before Hermes SDK, we also call this type `Tagged`, as it is tagging a `Value` with a `Tag`. To shorten its representation, we also use the Greek symbol ω to represent `Field`.

Using `Field`, we can now more concisely represent the generic field type for `Person` as `Cons<Field<symbol!("name"), String>, Cons<Field<symbol!("age"), u8>, Cons<Field<symbol!("address"), String>, Nil>>>`. As we can see, the type representation becomes quite more cluttered once we include the field tags. We can slightly shorten the type with the Greek alphabets into `π<ω<symbol!("name"), String>, π<ω<symbol!("age"), u8>, π<ω<symbol!("address"), String>, ε>>>`.

It is worth noting that the `Field` type is not strictly necessary for most of the generic implementation. However, CGP macros such as `HasFields` would wrap values around `Field`, so that it is easier to recover the original data structure.

## Indexed Types

Aside from struct with named fields, Rust also supports structs with position-based fields, such as:

```rust
pub struct Person(String, u8);
```

When representing the struct as generic fields, we would use the `Index` type to identify the field tag also by _position_ instead of by name:

```rust
pub struct δ<const I: usize>;

pub use δ as Index;
```

Similar to `Char`, the `Index` type makes use of const generics to lift a `usize` value as type. With that, a type like `Index<0>` is different from the type `Index<1>`. To shorten its representation, we use the Greek alphabet δ to represent `Index`.

With the `Index` type, we can now have a generic field representation of `Person` as `Cons<Field<Index<0>, String>, Cons<Field<Index<1>, u8>, Nil>>`, or `π<ω<δ<0>, String>, π<ω<δ<1>, u8>, ε>>` in the shortened representation.

## `HasFields` Trait

CGP provides a `HasFields` trait that can be derived to convert a data type into its generic representation:

```rust
pub trait HasFields {
    type Fields;
}
```

We can use `#[derive(HasFields)]` on the `Person` struct we have earlier, such as:

```rust
#[derive(HasFields)]
pub struct Person {
    pub name: String,
    pub age: u8,
    pub address: String,
}
```

which would generate the following implementation:

```rust
impl HasFields for Person {
    type Fields = π<ω<symbol!("name"), String>, π<ω<symbol!("age"), u8>, π<ω<symbol!("address"), String>, ε>>>;
}
```

## `FromFields` and `ToFields` Trait

CGP also provides `FromFields` and `ToFields` traits that allows for conversion of a data type to/from its field representation:

```rust
pub trait FromFields: HasFields {
    fn from_fields(fields: Self::Fields) -> Self;
}```rust
impl HasFields for Person {
    type Fields = π<ω<symbol!("name"), String>, π<ω<symbol!("age"), u8>, π<ω<symbol!("address"), String>, ε>>>;
}
```

pub trait ToFields: HasFields {
    fn to_fields(self) -> Self::Fields;
}
```

These traits are also implemented automatically when using `#[derive(HasFields)]`. All you need to know is that we can use the `from_fields` and `to_fields` methods to convert any struct into its generic fields.

## `HasFieldsRef` Trait

The `HasFields` trait, together with `ToFields` requires ownership over the value, and performs destructive destructuring. In case if we want to keep the original struct, we can use the `HasFieldsRef` trait, which _borrows_ the field values from the original struct:

```rust
pub trait HasFieldsRef {
    type FieldsRef<'a>
    where
        Self: 'a;
}
```

Since the field values are borrowed, the `FieldsRef` associated type is parameterized by an additional lifetime `<'a>`.

The `HasFieldsRef` trait is also automatically derived by `#[derive(HasFields)]`. So the example `Person` struct earlier would also derive the following implementation:

```rust
impl HasFieldsRef for Person {
    type Fields<'a> = π<ω<symbol!("name"), &'a String>, π<ω<symbol!("age"), &'a u8>, π<ω<symbol!("address"), &'a String>, ε>>>;
}
```

As we can see, each of the borrowed field in `HasFieldsRef` contain a `&'a` reference to the original field values.