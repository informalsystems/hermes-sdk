
# Context-Generic Programming (CGP)

Hermes SDK makes heavy use of a new programming paradigm,
[context-generic programming](https://contextgeneric.dev/) (CGP),
which is developed by us to implement the relayer as a collection of loosely-coupled
components. At a high level, CGP allows code to be written to be generic over a
context type, i.e. the type that is used as `Self`. With that, we can easily define
multiple context types that re-use the same context-generic code through minimal wiring.

For readers with object-oriented programming (OOP) background, CGP shares some
similarities with advanced OOP concepts, such as mixins and dependency injection.
The main difference is that CGP offers polymorphism at _compile time_, with no
dynamic dispatch involved at runtime. With the use of associated types, CGP also
allows strongly-typed relations to be established between different types, thus
achieving type safety without any use of upcast or downcast operation.

In Hermes SDK, we make use of the [`cgp`](https://github.com/contextgeneric/cgp)
crate to define context-generic components. To learn more about CGP, readers are
encouraged to read the in-progress book,
[Context-Generic Programming Patterns](https://patterns.contextgeneric.dev/).
