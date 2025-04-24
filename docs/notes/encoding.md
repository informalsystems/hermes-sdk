# Encoding Framework

Hermes SDK uses a CGP-based encoding framework to provide a modular and composable way of handling encoding of data types across different encoding schemes. In this document, we will walk through a high level overview of how the encoding works.

## Problem Statement

As a chain client, Hermes SDK needs to support encoding of data types across different encoding schemes, including Protobuf, Cairo, JSON, and Borsh. Furthermore, the encoding can sometimes come with different "flavors", such as wrapping a protobuf-encoded payload inside an `Any`, and then encoding the payload again.

More generally, encoding schemes can sometimes interleave with each other, such as encoding a value with JSON, putting it inside a wrapper that is then encoded with Protobuf.

Traditionally, different encodings tend to be implemented as separate libraries, each having their own ways of designing the APIs. This result in a lot of ad hoc code written to call each encoding library in their own convention. Furthermore, the encoding libraries tend to require additional encoding traits, such as `Serialize` or `BorshSerialize`, to be derived on a data type. This makes it challenging to define domain types that can be usable with all encodings, as the domain types would then become bloated with derivations and dependencies that may not necessarily be needed by everyone.

The CGP approach used in Hermes SDK offers a universal interface to support all encodings the same way. We will next look at how this is done.

## Encoding Context

Encodings in Hermes SDK are implemented as encoding contexts. The context makes use of CGP to wire up context-generic encoding providers, to form an encoding implementation for a specific chain.

Most of the time, the decision of defining multiple encoding contexts depends on whether each context requires different wirings. Unlike general encoding libraries, an encoding context can usually encode specific data types that have been wired with it. For example, the `CosmosEncoding` context knows how to encode a `TendermintClientState` with protobuf, but it won't be able to encode external types that have not been wired, such as `EthereumClientState`.

As a result, if some external code needs to support new types such as `EthereumClientState`, they would not able to reuse existing encoding contexts such as `CosmosEncoding`. However, we could define a new encoding context such as `CosmosEthereumEncoding`, and make it _extend_ from existing encoding [presets](./presets.md) that are shared with `CosmosEncoding`.

Usually, an encoding context can consist of an empty struct with no field. This is because most of the encodings do not require additional metadata to perform the encoding or decoding. However, sometimes an encoding may require additional metadata, such as schema information or contract addresses, in order for the encoding to be done correctly. In these cases, the encoding context may contain additional fields that can be used by the encoding providers through dependency injection.

## `HasEncoding`

Typically, the encoding context is embedded in another parent context, such as the chain context. We opt for a compositional approach that separates the encoding context from the main context, because that allows the encoding to be obtained and used without full access to the parent context. For example, when constructing relay messages that contain counterparty types, such as `Counterparty::ClientState`, we would need to access the counterparty encoding context, `Counterparty::Encoding`, to perform the encoding for us. However, since we don't want to expose the whole counterparty chain context, we can still get only the counterparty encoding context through other ways.

When possible, we will obtain a reference of the encoding context through the parent context:

```rust
pub trait HasEncodingType<Kind>: Async {
    type Encoding: Async;
}

pub trait HasEncoding<Kind>: HasEncodingType<Kind> {
    fn encoding(&self) -> &Self::Encoding;
}
```

The `HasEncodingType` trait defines an abstract `Encoding` context type provided by the parent context. It is parameterized by a `Kind` tag, so that the parent context can provide more than one encoding context with different tags.

The `HasEncoding` trait provides a getter method for getting a reference of the encoding context through the parent context. It accepts a `&self` parameter, thus getting the encoding context requires access to the parent context.

## `HasDefaultEncoding`

There is also an alternative version of the accessor, `HasDefaultEncoding`, which is defined as follows:

```rust
pub trait HasDefaultEncoding<Kind>: HasEncodingType<Kind, Encoding: 'static> {
    fn default_encoding() -> &'static Self::Encoding;
}
```

Compared to `HasEncoding`, `HasDefaultEncoding` do not require a `&self` parameter to get the encoding context. In other words, we can think of there being a "singleton" or "global" encoding context that can always be accessed anywhere.

To implement `HasDefaultEncoding`, the encoding context typically needs to be an empty struct, so that it can be trivally constructed at compile time. This also means that the encoding context cannot depend on additional values, such as contract classes.

The `HasDefaultEncoding` trait is mainly used to get the counterparty encoding context, from chain providers that only have access to the local chain context.

## `HasEncodedType`

```rust
pub trait HasEncodedType: Async {
    type Encoded: Async;
}
```

The `HasEncodedType` trait defines an abstract `Encoded` type that would represent the encoded form of values. Typically, this would be just raw bytes, i.e. `Vec<u8>`. However, some encoding contexts may encode values into something other than bytes, such as `Vec<char>` or `Vec<Felt>`.

We can also define custom `Encoded` types that contain more structure than just raw bytes, such as `serde_json::Value` or `Event`. The advantage with this is that we get to use the same encoding interfaces to encode values into more complex types, not just raw bytes.

More generally, there are a lot of similarity between building encoders and _parsers_. In fact, a lot of the techniques we used for encoding are inspired by _parser combinators_. As a result, it would also make sense to instantiate `Encoded` types with abstract syntax trees such as `TokenStream`.

The main takeaway here is that, while it is common to use `Vec<u8>` as the `Encoded` type, you don't need to get surprised when you see other `Encoded` types being defined in Hermes SDK code bases.

## `CanEncode` and `CanDecode`

```rust
pub trait CanEncode<Strategy, Value>: HasEncodedType + HasErrorType {
    fn encode(&self, value: &Value) -> Result<Self::Encoded, Self::Error>;
}

pub trait CanDecode<Strategy, Value>: HasEncodedType + HasErrorType {
    fn decode(&self, encoded: &Self::Encoded) -> Result<Value, Self::Error>;
}
```

The `CanEncode` and `CanDecode` traits are the simplest encoding traits that can be used with an encoding context. The traits are parameterized by a `Value` type, which is the type which would be encoded or decoded.

The traits are also parameterized by a `Strategy` tag type, which allows different versions of encoders for the same `Value` type to co-exist. The `Strategy` type only acts as a tag, and is typically named based on the encoding scheme used. For example, the `ViaProtobuf` strategy is used by convention to encode values into bytes following the Protobuf encoding format.

The encoding methods on these traits are pretty straightforward. The `encode` method takes a reference to `Value`, and tries to encode it into the `Encoded` form. In reverse, the `decode` method takes a reference to `Encoded`, and returns a new owned `Value` if the encoding is successful.

Based on the method signatures, we can see that the way the references are used means that the encoding may not always be done the most efficiently. For example, since `encode` borrows the `Value`, it may need to clone the value if it needs to make use of `From` to convert the value into some other form, before the actual encoding is done. Nevertheless, these traits serve as a good starting point for implementing naive encoders, without introducing too steep learning curves to the developers.

In Hermes SDK, the `CanEncode` and `CanDecode` traits are initially used as a glue layer to call the legacy implementation of protobuf encoding through the `ibc_proto` crate. We also use it to implement the `ViaAny` encoding strategy that wraps encoded values into the Protobuf `Any`, and then performs another round of encoding.

In newer encoding implementations, such as the protobuf encoding for `Height`, we make use of the more efficient `CanEncodeMut` and `CanDecodeMut` traits, which we will walk through next. But the main thing to note here is that existing `CanEncode` and `CanDecode` implementations are mostly legacy, and new encoding implementations should prefer the more efficient `CanEncodeMut` and `CanDecodeMut` traits.

## `CanEncodeMut` and `CanDecodeMut`

An additional source of inefficiency in the `CanEncode` trait is that the `Encoded` result is returned as an owned value. This can be problematic for lower level encoding implementations that need to perform multiple encoding operations. For example, we can implement `CanEncode<Strategy, (String, String)>` by calling `CanEncode<Strategy, String>` twice. But now we have to stitch together two separate `Vec<u8>`, which may involve expensive copying.

A more efficient approach would be to use a mutable _encode buffer_, which encoders and decoders can write onto before returning the final result.

```rust
pub trait HasEncodeBufferType {
    type EncodeBuffer: Default;
}

pub trait HasDecodeBufferType {
    type DecodeBuffer<'a>;
}
```

The `EncodeBuffer` is an abstract type representing the encode buffer. Typically, we can also use `Vec<u8>` as the encode buffer, but the different abstract types help prevent us from mixing up the two uses.

We also have a separate abstract `DecodeBuffer` type that is used for decoding. Compared to `EncodeBuffer`, the `DecodeBuffer` is also parameterized by a lifetime `'a`, meaning that it may contain references to offsets within the raw buffer. For example, this is used for the implementation of Protobuf decoding, where the `DecodeBuffer` is a list of byte slices that point to specific protobuf fields. Similarly, we can also use this to define specialized decode buffers for types such as JSON.


```rust
pub trait CanEncodeMut<Strategy, Value>: HasEncodeBufferType + HasErrorType {
    fn encode_mut(&self, value: &Value, buffer: &mut Self::EncodeBuffer)
        -> Result<(), Self::Error>;
}

pub trait CanDecodeMut<Strategy, Value>: HasDecodeBufferType + HasAsyncErrorType {
    fn decode_mut<'a>(&self, buffer: &mut Self::DecodeBuffer<'a>) -> Result<Value, Self::Error>;
}
```

Compared to the simple versions, the `CanEncodeMut` and `CanDecodeMut` traits accept mutable `EncodeBuffer` and `DecodeBuffer` that can be modified during the encoding process.

It is usually possible to define generic implementations of `CanEncode` and `CanDecode` that call `CanEncodeMut` and `CanDecodeMut`, and perform conversion between the `Encoded` abstract type and the `EncodeBuffer` and `DecodeBuffer` abstract types. As a result, once we have implemented `CanEncodeMut` and `CanDecodeMut` for a given `Value`, we can usually also get the `CanEncode` and `CanDecode` implementations for free.

## Encoder Composition

With CGP, the encoding framework in Hermes SDK allows highly generic encoding implementations to be defined and composed easily. We will now take a look at how encoding composition is done.

Typically, an encoding scheme like Protobuf would be made of manual implementation of primitive values, such as `u64` and `String`. Once the primitive encoders are implemented, we can then compose them to build higher level encoders for specific data types.

For example, given the Cosmos `Height` type:

```rust
#[derive(HasField, HasFields)]
pub struct Height {
    pub revision_number: u64,
    pub revision_height: u64,
}
```

We know that the encoding of this data type is consist of encoding two separate `u64` fields. So if the encoding context implements `CanEncodeMut<Strategy, u64>`, and if we know how to combine multiple encoded fields together, then we can derive the implementation of this data type automatically.

CGP provides two traits that can help us implement datatype-generic encoders. The `HasField` trait provides accessors to read the fields in a struct, while the `HasFields` trait provides conversion of data types into generic representation. For the example `Height` type, the following implementations are derived:

```rust
impl HasField<symbol!("revision_number")> for Height {
    type Value = u64;
}

impl HasField<symbol!("revision_height")> for Height {
    type Value = u64;
}

impl HasFields for Height {
    type Fields = Product![
        Field<symbol!("revision_number"), u64>,
        Field<symbol!("revision_height"), u64>,
    ];
}
```

Using the derived implementations, we can now reduce the problem of encoding a specific data type into encoding general product and sum types.

## Product Encoder

For simplicity, we will first look at how to encode a type like `Product![u64, u64]`. Recall that the expanded form is the type-level list `Cons<u64, Cons<u64, Nil>>`. For every step of the encoding, we want to consider the head of the list, and then the remainder.

We would define a `ProductEncoder` that satisfies the following constraints:

- `ProductEncoder: MutEncoder<Encoding, Strategy, Cons<u64, Cons<u64, Nil>>>`, if:
    - `Encoding: CanEncodeMut<Strategy, u64>`
    - `ProductEncoder: MutEncoder<Encoding, Strategy, Cons<u64, Nil>>`, if:
        - `Encoding: CanEncodeMut<Strategy, u64>`
        - `ProductEncoder: MutEncoder<Encoding, Strategy, Nil>`

To implement such an encoder, we first implement the terminal case, when the list is just `Nil`:

```rust
impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, Nil> for ProductEncoder
```

After that, we implement the recursive case for processing the head of a list:

```rust
impl<Encoding, Strategy, Value, Tail> MutEncoder<Encoding, Strategy, Cons<Value, Tail>> for ProductEncoder
where
    Encoding: CanEncodeMut<Strategy, Value>,
    ProductEncoder: MutEncoder<Encoding, Strategy, Tail>,
    ...
```

We can see that this generic implementation could be used for the list we have:

- `Cons<u64, Cons<u64, Nil>>` would have `Value = u64` and `Tail = Cons<u64, Nil>`
- `Cons<u64, Nil>` would have `Value = u64` and `Tail = Nil`

## Encoder Product

The product encoder we went through earlier is a single encoder provider that encodes a heterogeneous list of values. It is also possible for us to define a list of _encoders_ that work on a single value. This kind of encoder can for example use the `HasField` trait to read values directly from a data type, and then perform the encoding.

We will show an example encoder product in action. We can first define a provider `CombineEncoders<Encoders>`, with `Encoders` being the heterogenous list of encoders. An example encoder for `Height` would be

```rust
CombineEncoders<Product![
    EncodeField<symbol!("revision_number")>,
    EncodeField<symbol!("revision_height")>,
]>
```

which would be expanded into:

```rust
CombineEncoders<
    Cons<
        EncodeField<symbol!("revision_number")>,
        Cons<
            EncodeField<symbol!("revision_height")>,
            Nil,
        >,
    >
>
```

The implementation of `CombineEncoders` would need to have the following constraints satisfied to implement the combined encoder:

- `CombineEncoders<Cons<EncodeField<symbol!("revision_number")>, Cons<EncodeField<symbol!("revision_height")>,Nil>>>: MutEncoder<Encoding, Strategy, Value>` if:
    - `EncodeField<symbol!("revision_number")>: MutEncoder<Encoding, Strategy, Value>`
    - `CombineEncoders<Cons<EncodeField<symbol!("revision_height")>,Nil>>: MutEncoder<Encoding, Strategy, Value>` if:
        - `EncodeField<symbol!("revision_height")>: MutEncoder<Encoding, Strategy, Value>`
        - `CombineEncoders<Nil>: MutEncoder<Encoding, Strategy, Value>`

Compared to the example `ProductEncoder` we had earlier, we can see that the list being processed is the encoders, while the `Value` being encoded remains the same in each step.

To implement `CombineEncoders`, we would start with the terminal case of `Nil`:

```rust
impl<Encoding, Strategy, Value> MutEncoder<Encoding, Strategy, Value> for CombineEncoders<Nil>
```

We can then implement the inductive case to process the head of an encoder list:

```rust
impl<Encoding, Strategy, Value, Encoder, Tail> MutEncoder<Encoding, Strategy, Value>
    for CombineEncoders<Cons<Encoder, Tail>>
where
    Encoder: MutEncoder<Encoding, Strategy, Value>,
    CombineEncoders<Tail>: MutEncoder<Encoding, Strategy, Value>,
```

## Higher Order Encoders

The `CombineEncoders` provider that we have seen previously makes use of _higher order encoders_, meaning that it takes in other encoders in its generic parameters, instead of calling the context to perform the encoding. This allows for more flexibility, as we don't necessarily need to wire up all encoders used with the context.

The use of higher order encoder is also important for supporting Protobuf encoding, which requires additional tag value to be specified for each field. This means that the protobuf encoding context cannot have a general implementation like `CanEncodeMut<Strategy, u64>`, because we wouldn't be able to know which tag to use.
Instead, we would use specialized encoders like `EncodeU64ProtoField<1>` to include the tag value in the encoder itself to support the encoding. With higher order encoders, we will pass these encoders to the generic parameters of other encoders to form the final encoder.

Although higher order encoders are pretty flexible, we sometimes do want to just use the encoder provided by the context to perform the inner encoding. In such case, we can use the `UseContext` encoder, which just calls the encoding method from the context:

```rust
impl<Encoding, Strategy, Value> MutEncoder<Encoding, Strategy, Value> for UseContext
where
    Encoding: CanEncodeMut<Strategy, Value>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &Value,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        encoding.encode_mut(value, buffer)
    }
}
```

When possible, we can also set `UseContext` as the default generic parameter, so that when no generic parameter is specified, it is automatically used and thus reduce the boilerplate for the default case.

## Proxy Data Types

When data types are defined in Hermes SDK code, we can make use of `#[derive(HasField, HasFields)]` to make use of datatype-generic encoders to help us implement the encodings. However, when the data types are not owned by Hermes SDK, such as for ibc-rs types, it would make the encoding implementation more complicated.

Furthermore, external types such as those from ibc-rs may contain private fields or fallible constructors. That makes the encoding implementation non-trivial, thus requiring explicit implementation of traits like `MutEncoder` for each external type.

To simplify the manual implementation of the encoder traits, we can make use of _proxy_ data types that can derive `HasFields` to help us perform the actual encoding, once we obtain the raw values from the external types.

Following shows an example of how we can implement `MutEncoder` for the `Height` type defined in ibc-rs:

```rust
use ibc::Height;

#[derive(HasField, HasFields)]
pub struct RawHeight {
    pub revision_number: u64,
    pub revision_height: u64,
}

#[cgp_new_provider(MutEncoderComponent)]
impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, Height> for EncodeIbcHeight
where
    Encoding: CanEncodeMut<Strategy, RawHeight>,
{

    fn encode_mut(
        encoding: &Encoding,
        height: &Height,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        let revision_number = height.revision_number();
        let revision_height = height.revision_height();

        let raw_height = RawHeight {
            revision_number,
            revision_height,
        };

        encoding.encode_mut(raw_height, buffer)
    }
}
```

In the above example, we define a `RawHeight` data type that implements `HasFields`. We can then use the existing encoding providers to implement the encoding for `RawHeight`. Inside the `MutEncoder` implementation for `Height`, we then first try to extract or convert the values from `Height` into `RawHeight`, and then perform the encoding on that.

Through experience, we find that this approach of handling encoding for external data types provides the best developer experience, especially for developers who are new to the code base. As a result, it is recommended for new code to follow this pattern.

In the existing encoding code base, you may also find other approaches of doing the same thing as the proxy data type. For example, instead of defining a new proxy type `RawHeight`, existing implementations may also use types like `(u64, u64)` or `Product![u64, u64]` as the proxy type. While the approach also works, it could potentially lead to more confusion, as it may not be obvious for newcomers that a type like `Product![u64, u64]` is _isomorphic_ to the type `RawHeight`, meaning that they are essentially equivalent.

## Converter Encoders

The existing code base also try to define _converter_ traits to try to make the conversion between the external types and the proxy data types slightly simpler. The idea is that, we can see that the earlier `MutEncoder` for the ibc-rs `Height` type contains some boilerplate. Essentially, every one of such implementation involves a conversion between a domain type (e.g. `Height`) and a proxy type (e.g. `RawHeight`).

We may want to define some kind of converter encoder that would just perform the conversion using another trait, and then perform the encoding for us. This can be using the standard traits such as `From` or `TryFrom`. You may also see custom traits like `Transformer` that are basically more specific versions of `TryFrom`.

However in practice, we find that the savings achieved by such converter encoders to be not that much, and it only leads to more confusion with the additional layer of abstractions. To make the code easier to understand, we encourage migrating these existing code to implement the encoding traits directly. The existing code also tend to perform conversion to/from generic product and sum types, which could add to the confusion. When migrating the existing code, one should also define new proxy data types, and follow the example use of `RawHeight` to perform the conversion inside the encoder implementations.


## Future Improvements

The encoding framework in Hermes SDK has only reached an MVP status, and it is not very well polished at the current state. Technically, the encoding problem is a more general problem than building a cross-chain relayer. We also do not have much room or priority to iterate on the encoding design.

On the other hand, the encoding approaches we used in Hermes SDK will likely evolve externally and become a generalized CGP library for encoding. This may happen in few months time, and we may come up with more polished design with potential breaking changes.

However, given the current status of the project, the developers for Hermes SDK may not have the capacity to migrate to use this improved encoding framework. This means that whatever that is currently designed and implemented in Hermes SDK will likely stay, even when better alternatives become available.

This section mainly serves as a historical note for future readers, in case you wondered why Hermes SDK is having its own encoding framework even when the new CGP encoding framework is released. If the future team is adventurous enough, I hope that this document will help the team to migrate to any new approaches that are offered by the CGP project.
