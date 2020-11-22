
extern crate heck;
extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

mod util;

#[proc_macro_derive(EnumStr)]
pub fn self_tokenize(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let source = input.to_string();
    let ast = syn::parse_derive_input(&source).unwrap();
    let expanded = util::expand_macro(&ast, "");
    expanded.parse().unwrap()
}

#[proc_macro_derive(EnumStrCamelCase)]
pub fn self_tokenize_camel_case(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let source = input.to_string();
    let ast = syn::parse_derive_input(&source).unwrap();
    let expanded = util::expand_macro(&ast, "CamelCase");
    expanded.parse().unwrap()
}

#[proc_macro_derive(EnumStrKebabCase)]
pub fn self_tokenize_kebab_case(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let source = input.to_string();
    let ast = syn::parse_derive_input(&source).unwrap();
    let expanded = util::expand_macro(&ast, "KebabCase");
    expanded.parse().unwrap()
}

#[proc_macro_derive(EnumStrMixedCase)]
pub fn self_tokenize_mixed_case(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let source = input.to_string();
    let ast = syn::parse_derive_input(&source).unwrap();
    let expanded = util::expand_macro(&ast, "MixedCase");
    expanded.parse().unwrap()
}

#[proc_macro_derive(EnumStrShoutySnakeCase)]
pub fn self_tokenize_shouty_snake_case(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let source = input.to_string();
    let ast = syn::parse_derive_input(&source).unwrap();
    let expanded = util::expand_macro(&ast, "ShoutySnakeCase");
    expanded.parse().unwrap()
}

#[proc_macro_derive(EnumStrSnakeCase)]
pub fn self_tokenize_snake_case(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let source = input.to_string();
    let ast = syn::parse_derive_input(&source).unwrap();
    let expanded = util::expand_macro(&ast, "SnakeCase");
    expanded.parse().unwrap()
}

#[proc_macro_derive(EnumStrTitleCase)]
pub fn self_tokenize_title_case(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let source = input.to_string();
    let ast = syn::parse_derive_input(&source).unwrap();
    let expanded = util::expand_macro(&ast, "TitleCase");
    expanded.parse().unwrap()
}
