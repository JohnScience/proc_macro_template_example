// Consider removing the attributes below after prototyping
#![allow(unused_imports)]
#![allow(unused_variables)]

use quote::quote;
use proc_macro2 as pm2;

pub fn my_fnlike_macro(input: pm2::TokenStream) -> pm2::TokenStream {
    todo!()
}

pub fn my_derive_macro_wo_attrs(input: pm2::TokenStream) -> pm2::TokenStream {
    todo!()
}

pub fn my_derive_macro_w_attrs(input: pm2::TokenStream) -> pm2::TokenStream {
    todo!()
}

pub fn my_attr_macro(attr: pm2::TokenStream, item: pm2::TokenStream) -> pm2::TokenStream {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn assert_tokens_eq(expected: &pm2::TokenStream, actual: &pm2::TokenStream) {
        let expected = expected.to_string();
        let actual = actual.to_string();
    
        if expected != actual {
            println!(
                "{}",
                colored_diff::PrettyDifference {
                    expected: &expected,
                    actual: &actual,
                }
            );
            println!("expected: {}", &expected);
            println!("actual  : {}", &actual);
            panic!("expected != actual");
        }
    }
    
    #[test]
    pub fn test_my_fnlike_macro() {
        let before = quote! {
            // Function-like macro input. Nearly arbitrary AST.
        };

        let expected = quote! {
            // Function-like macro output. The invocation is replaced with this.
        };

        let actual = my_fnlike_macro(before);

        assert_tokens_eq(&actual, &expected);
    }
    
    #[test]
    pub fn test_my_derive_macro_wo_attrs() {
        let before = quote! {
            // Derive macro input: struct, enum, or union.
        };

        let expected = quote! {
            // Derive macro output. It is appended to the resulting AST.
        };

        let actual = my_derive_macro_wo_attrs(before);

        assert_tokens_eq(&actual, &expected);
    }
    
    #[test]
    pub fn test_my_derive_macro_w_attrs() {
        let before = quote! {
            // Derive macro input: struct, enum, or union.
        };

        let expected = quote! {
            // Derive macro output. It is appended to the resulting AST.
        };

        let actual = my_derive_macro_w_attrs(before);

        assert_tokens_eq(&actual, &expected);
    }
    
    #[test]
    pub fn test_my_attr_macro() {
        let attr = quote! {
            // Attribute macro input. For example, `target_os = "linux"` in
            // `#[cfg(target_os = "linux")]`.
            // https://doc.rust-lang.org/reference/attributes.html
        };

        let item = quote! {
            // Attribute macro input: an item.
            // https://doc.rust-lang.org/reference/items.html
        };

        let expected = quote! {
            // Attribute macro output. It replaces the input item.
        };

        let actual = my_attr_macro(attr, item);

        assert_tokens_eq(&actual, &expected);
    }
    }