# Encoding Framework

Hermes SDK uses a CGP-based encoding framework to provide a modular and composable way of handling encoding of data types across different encoding schemes. In this document, we will walk through a high level overview of how the encoding works.

## Problem Statement

As a chain client, Hermes SDK needs to support encoding of data types across different encoding schemes, including Protobuf, Cairo, JSON, and Borsh. Furthermore, the encoding can sometimes come with different "flavors", such as wrapping an protobuf-encoded payload inside an `Any`, and then encoding the payload again.

More generally, encoding schemes can sometimes interleave with each other, such as encoding a value with JSON, putting it inside a wrapper that is then encoded with Protobuf.

Traditionally, different encodings tend to be implemented as separate libraries, each having their own ways of designing the APIs. This result in a lot of ad hoc code written to call each encoding library in their own convention. Furthermore, the encoding libraries tend to require additional encoding traits, such as `Serialize` or `BorshSerialize`, to be derived on a data type. This makes it challenging to define domain types that can be usable with all encodings, as the domain types would then become bloated with derivations and dependencies that may not necessary be needed by everyone.

The CGP approach used in Hermes SDK offers a universal interface to support all encodings the same way. We will next look at how this is done.

## Encoding Context

Encodings in Hermes SDK are implemented as encoding contexts. The context makes use of CGP to wire up context-generic encoding providers, to form an encoding implementation for a specific chain.

Most of the type, the decision of defining multiple encoding contexts depends on whether each context requires different wirings. Unlike general encoding libraries, an encoding context can usually encode specific data types that have been wired with it. For example, the `CosmosEncoding` context knows how to encode a `TendermintClientState` with protobuf, but it won't be able to encode an external type that have not been wired, such as `EthereumClientState`.

As a result, if some external code need to support new types such as `EthereumClientState`, they would not able to reuse existing encoding contexts such as `CosmosEncoding`. However, they could define a new encoding context such as `CosmosEthereumEncoding`, and make it extend from existing encoding [presets](./presets.md) that are shared with `CosmosEncoding`.

Most of the time, an encoding context can be consist of an empty struct with no field. This is because most of the encodings do not require additional metadata to perform the encoding or decoding. However, sometimes an encoding may require additional metadata, such as schema information or contract addresses, in order for the encoding to be done correctly. In these cases, the encoding context may contain additional fields that can be used by the encoding providers through dependency injection.

## `HasEncoding`

Typically, the encoding context is embedded in another context, such as the chain context. We opt for a compositional approach that separates the encoding context from the main context, because that allows the encoding to be obtained and used without full access to the parent context. For example, when constructing relay messages that contain counterparty types, such as `Counterparty::ClientState`, we would need to access the counterparty encoding context, `Counterparty::Encoding`, to perform the encoding for us. However, since we don't want to expose the whole counterparty chain context, we can still get only the counterparty encoding context through other ways.

When possible, we will obtain a reference of the encoding context through the parent context:

```rust
#[cgp_type {
    name: EncodingTypeProviderComponent<Kind>
}]
pub trait HasEncodingType<Kind>: Async {
    type Encoding: Async;
}

#[cgp_component {
    name: EncodingGetterComponent<Kind>,
    provider: EncodingGetter,
}]
pub trait HasEncoding<Kind>: HasEncodingType<Kind> {
    fn encoding(&self) -> &Self::Encoding;
}
```

The `HasEncodingType` trait defines an abstract `Encoding` context type provided by the parent context. It is parameterized by a `Kind` tag, so that the parent context can provide more than one encoding context with different tags.

The `HasEncoding` trait provides a getter method for getting a reference of the encoding context through the parent context. It accepts a `&self` parameter, thus getting the encoding context requires access to the parent context.

## `HasDefaultEncoding`

There is also an alternative version of the accessor, `HasDefaultEncoding`, which is defined as follows:

```rust
#[cgp_component {
    name: DefaultEncodingGetterComponent<Kind>,
    provider: DefaultEncodingGetter,
}]
pub trait HasDefaultEncoding<Kind>: HasEncodingType<Kind, Encoding: 'static> {
    fn default_encoding() -> &'static Self::Encoding;
}
```

Compared to `HasEncoding`, `HasDefaultEncoding` do not require a `&self` parameter to get the encoding context. In other words, we can think of there being a "singleton" or "global" encoding context that can always be accessed anywhere.

To implement `HasDefaultEncoding`, the encoding context typically needs to be an empty struct, so that it can be trivally be constructed at compile time. This also means that the encoding context cannot depend on additional values, such as contract classes.

The `HasDefaultEncoding` trait is mainly used to get the counterparty encoding context, from chain providers that only have access to the local chain context.

## `HasEncodedType`

```rust
pub trait HasEncodedType: Async {
    type Encoded: Async;
}
```

The `HasEncodedType` trait defines an abstract `Encoded` type that would represent the encoded form of values. Typically, this would be just raw bytes, i.e. `Vec<u8>`. However, some encoding contexts may encode values into something other than bytes, such as `Vec<char>` or `Vec<Felt>`.

We can also define custom `Encoded` types that contain more structure than just raw bytes, such as `serde_json::Value` or `Event`. The advantage with this is that we get to use the same encoding interfaces to encode values into more complex types, not just raw bytes.

More generally, there are a lot of similarity between building encoders and _parsers_. In fact, a lot of the techniques we used for encoding are inspired from _parser combinators_. As a result, it would also make sense to instantiate `Encoded` types with abstract syntax trees such as `TokenStream`.

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

The encoding methods on these traits are pretty straightforward. The `encode` method takes a reference to `Value`, and try to encode it into the `Encoded` form. In reverse, the `decode` method takes a reference to `Encoded`, and returns a new owned `Value` if the encoding is successful.

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

Compared to the simple versions, the `CanEncodeMut` and `CanDecodeMut` traits accept mutable `EncodeBuffer` and `DecodeBuffer` thtat can be modified during the encoding process.