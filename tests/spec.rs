
extern crate djed_enum_str_derive;

use djed_enum_str_derive::{EnumStrCamelCase, EnumStrKebabCase, EnumStrMixedCase};

#[derive(EnumStrCamelCase)]
pub enum Unit {
    Foo,
    BarBaz
}

#[derive(EnumStrKebabCase)]
pub enum Tuple {
    Foo(String),
    BarBaz(Vec<String>)
}

#[derive(EnumStrMixedCase)]
pub enum Struct {
    Foo { value: String },
    BarBaz { value: Vec<String> }
}

#[test]
fn test_1() {
    let foo = Unit::Foo;
    assert_eq!(foo.as_ref(), "Foo");

    let bar = Unit::BarBaz;
    assert_eq!(bar.as_ref(), "BarBaz");
}

#[test]
fn test_2() {
    let foo = Tuple::Foo("heck".to_string());
    assert_eq!(foo.as_ref(), "foo");

    let bar = Tuple::BarBaz(vec!["heck".to_string()]);
    assert_eq!(bar.as_ref(), "bar-baz");
}

#[test]
fn test_3() {
    let foo = Struct::Foo {
        value: "heck".to_string()
    };
    assert_eq!(foo.as_ref(), "foo");

    let bar = Struct::BarBaz {
        value: vec!["heck".to_string()]
    };
    assert_eq!(bar.as_ref(), "barBaz");
}
