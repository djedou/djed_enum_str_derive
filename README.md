# djed_enum_str_derive


A procedural macro derive allowing for automatically implementing `AsRef<str>` for enums.

## Purpose
Automate the tedious process of stringifying enum variants where needed.

## How to use

Add this to your `Cargo.toml` file:

```toml
[dependencies]
djed_enum_str_derive = { git = "https://github.com/victorporof/djed_enum_str_derive.git" }
```

Then, simply import the library into your code and derive the `EnumStr` trait on your data structures.

Available derives:
* `EnumStr`
* `EnumStrCamelCase`
* `EnumStrKebabCase`
* `EnumStrMixedCase`
* `EnumStrShoutySnakeCase`
* `EnumStrSnakeCase`
* `EnumStrTitleCase`

```rust
#![feature(proc_macro)]
extern crate djed_enum_str_derive;

use djed_enum_str_derive::{EnumStrSnakeCase};

#[derive(EnumStrCamelCase)]
enum Data {
    Foo,
    BarBaz
}

assert_eq!(Data::Foo.as_ref(), "foo");
assert_eq!(Data::BarBaz.as_ref(), "bar-baz");
```
