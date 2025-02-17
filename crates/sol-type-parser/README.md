# alloy-sol-type-parser

Solidity type string parsing.

We do not recommend most users use this crate directly. This library provides a
simple parser for working with Solidity type encoded as strings. It is
primarily a dependency for the user-facing APIs in [`alloy-json-abi`] and
[`alloy-dyn-abi`]. Please see the documentation for those crates for
more information.

This parser generally follows the [solidity spec], however, it supports only a
subset of possible types, chosen to support ABI coding.

[solidity spec]: https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.typeName
[`alloy-json-abi`]: https://docs.rs/alloy-json-abi/latest/alloy_json_abi/
[`alloy-dyn-abi`]: https://docs.rs/alloy-dyn-abi/latest/alloy_dyn_abi/

### Usage

The `TypeSpecifier` is the top-level type in this crate. It is a wrapper around
a section of a string (called a `span`). It progressively breaks the strings
down into subspans, and adds metadata about the type. E.g. it tracks the stem
type as well as the sizes of array dimensions. A `TypeSpecifier` is expected to
handle any valid Solidity type string.

```rust
use alloy_sol_type_parser::TypeSpecifier;
use core::num::NonZeroUsize;

// Parse a type specifier from a string
let my_type = TypeSpecifier::try_from("uint8[2][]").unwrap();

// Read the total span
assert_eq!(
    my_type.span(),
    "uint8[2][]"
);

// A type specifier has a stem type. This is the type string, stripped of its
// array dimensions.
assert_eq!(my_type.stem.span(), "uint8");

// Arrays are represented as a vector of sizes. This allows for deep nesting.
assert_eq!(
    my_type.sizes,
    // `None` is used for dynamic sizes. This is equivalent to `[2][]`
    vec![NonZeroUsize::new(2), None]
);

// Type specifiers also work for complex tuples!
let my_tuple = TypeSpecifier::try_from("(uint8,(uint8[],bool))[39]").unwrap();
assert_eq!(
    my_tuple.stem.span(),
    "(uint8,(uint8[],bool))"
);

// Types are NOT resolved, so you can parse custom structs just by name.
let my_struct = TypeSpecifier::try_from("MyStruct").unwrap();
```

### Why not support `parse()`?

The `core::str::FromStr` trait is not implemented for `TypeSpecifier` because
of lifetime constraints. Unfortunately, it is impossible to implement this for
a type with a lifetime dependent on the input str. Instead, we recommend using
`TryFrom::try_from`.

### Why not use `syn`?

This is NOT a full syntax library, and is not intended to be used as a
replacement for [`alloy-syn-solidity`]. This crate is intended to be used for
parsing type strings present in existing ecosystem tooling, and nothing else.
It is not intended to be used for parsing Solidity source code.

This crate is useful for:

- syntax-checking abi json files
- providing known-good input to [`alloy-dyn-abi`]
- porting ethers.js code to rust

It is NOT useful for:

- parsing Solidity source code
- generating rust code from Solidity source code
- generating Solidity source code from rust code

[`alloy-syn-solidity`]: https://docs.rs/alloy-syn-solidity/latest/alloy_syn_solidity/
