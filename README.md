Convert number to enum
======================

[![Build Status](https://api.travis-ci.org/dtolnay/enumn.svg?branch=master)](https://travis-ci.org/dtolnay/enumn)
[![Latest Version](https://img.shields.io/crates/v/enumn.svg)](https://crates.io/crates/enumn)
[![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/enumn)

This crate provides a derive macro to generate a function for converting a
primitive integer into the corresponding variant of an enum.

The generated function is named `n` and has the following signature:

```rust
impl YourEnum {
    pub fn n(value: Repr) -> Option<Self>;
}
```

where `Repr` is an integer type of the right size as described in more
detail below.

## Example

```rust
extern crate enumn;

#[derive(PartialEq, Debug, enumn::N)]
enum Status {
    LegendaryTriumph,
    QualifiedSuccess,
    FortuitousRevival,
    IndeterminateStalemate,
    RecoverableSetback,
    DireMisadventure,
    AbjectFailure,
}

fn main() {
    let s = Status::n(1);
    assert_eq!(s, Some(Status::QualifiedSuccess));

    let s = Status::n(9);
    assert_eq!(s, None);
}
```

## Signature

The generated signature depends on whether the enum has a `#[repr(..)]`
attribute. If a `repr` is specified, the input to `n` will be required to be
of that type.

```rust
#[derive(enumn::N)]
#[repr(u8)]
enum E {
    /* ... */
}

// expands to:
impl E {
    pub fn n(value: u8) -> Option<Self> {
        /* ... */
    }
}
```

On the other hand if no `repr` is specified then we get a signature that is
generic over a variety of possible types.

```rust
impl E {
    pub fn n<REPR: Into<i64>>(value: REPR) -> Option<Self> {
        /* ... */
    }
}
```

## Discriminants

The conversion respects explictly specified enum discriminants. Consider
this enum:

```rust
#[derive(enumn::N)]
enum Letter {
    A = 65,
    B = 66,
}
```

Here `Letter::n(65)` would return `Some(Letter::A)`.

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
