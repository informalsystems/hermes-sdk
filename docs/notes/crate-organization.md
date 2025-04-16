# Crate Organization

The Hermes SDK code base follows an unconventional approach of organizing crates. This can lead to confusion to people who are new to the code base, as they won't understand which crate to look at when reading or writing code.

This document gives a high level explanation of how the Hermes SDK crates are organized, and the constraints you should look out for.

## Levels

Hermes SDK crates are organized at multiple levels, going from fully abstract to full concrete code. The crates at each level may depend on crates at the same level or lower levels, but they should never depend on higher level crates. This forms an acyclic dependency graph, which is also required by Cargo as cyclic crate dependencies are not allowed.

There are 3 general levels of Hermes SDK crates:

- **Fully Abstract Core** - These crates only contain abstract type and method definitions, and have almost no dependency to other non-CGP crates or non-abstract crates. The crates might not even depend on `std`, i.e. allowing them to be used in no_std. Examples include `hermes-relayer-components` and `hermes-encoding-components`.

- **Context-Generic Providers** - These crates contain implementation specific providers, but do not contain any concrete context definitions. In other words, all providers in the crate are forced to be context-generic, which in turns allows them to be used in more than one concrete contexts. Examples include `hermes-cosmos-chain-components` and `hermes-protobuf-encoding-components`.

- **Concrete Contexts** - These crates contain the concrete context definitions, and perform the wiring using the context-generic providers from the crates from the previous level. The crates may also implement context-specific providers, i.e. providers that only work with the specific concrete contexts. Examples include `hermes-cosmos-relayer` and `hermes-integration-tests`.

As we can see, the way Hermes SDK crates depend on each other is the reverse of how typical Rust crates are organized. In non-CGP code, it is more common to first define concrete context types, and then define additional crates that import the concrete contexts from the base crates and add new functionalities. But with CGP, we are defining the concrete contexts _last_, and write most of our code in context-generic ways with minimal assumption about what the context should be.

In Hermes SDK, we create separate crates for different levels, so that it becomes clear that there is no way the code in a given abstract crate has access to the concrete types in its dependents. Doing so also force the developers to think harder, and find ways to write context-generic code without access to the concrete contexts.

## Writing Context-Specific Code

Although the best practices for Hermes SDK is to have separate crates for each level, we do not totally prevent writing context-specific code. If a developer prefers, they can always start writing context-specific code in the same crate that defines the concrete context, and then refactor that into context-generic code at a later time. This would especially help ease the onboarding process and learning curve, since they don't need to fully learn CGP all at once.

That said, the main constraint _all_ developers _must_ avoid is to add dependencies of crates that contain concrete context onto crates that only contain context-generic providers. For example, it is common for new-comers to attempt to add a crate like `hermes-cosmos-relayer` as a dependency for a crate like `hermes-cosmos-chain-components`, only to discover that resulting in cyclic dependency errors as `hermes-cosmos-relayer` itself depends on `hermes-cosmos-chain-components`.

The main confusion here is that the new developer sees that all the Cosmos-specific implementations are found in `hermes-cosmos-chain-components`, and therefore they also want to follow and add new implementations to that crate. But since they are new, the first instinct is to write context-specific code that works specifically with the `CosmosChain` context, which they then discover that they need to add `hermes-cosmos-relayer` as a dependency.

The right solution to resolve the conflict is that the new developer should add the context-specific implementation directly in the `hermes-cosmos-relayer` crate. Even though this is less than ideal, the trade off is acceptable if the main priority is in shipping code. Of course, there is a risk that eventually, a crate like `hermes-cosmos-relayer` could become so bloated with context-specific code it is preventing the same code to be reused for other use cases. But we could take action later to refactor the context-specific code to become context-generic, and move them back to `hermes-cosmos-chain-components` when that happens.

The main consideration here is that as long as there are strong boundaries between different crate levels, it will be clearer on which code is context-generic and which code is context-specific, which in turns make it easier to identify and refactor code at a later time.

When starting new sub-projects, it is also common to crate just a single crate that contains code from all levels. This approach is also acceptable if the developer prefers, but it is encouraged to split the crates later on when the amount of code become large.

The main risk of monolithic crate is that it becomes very easy to write context-specific code that is mixed within context-generic code. When that happens, it may become challenging to untangle and determine which code is reusable and which code is not. So if that happens often enough, the crate should be split to separate the code at different levels.