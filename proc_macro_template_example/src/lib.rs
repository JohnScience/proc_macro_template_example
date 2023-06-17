use proc_macro as pm;
use proc_macro2 as pm2;

#[proc_macro]
pub fn my_fnlike_macro(input: pm::TokenStream) -> pm::TokenStream {
    let input = pm2::TokenStream::from(input);
    let output = proc_macro_template_example_core::my_fnlike_macro(input);
    pm::TokenStream::from(output)
}

#[proc_macro_derive(MyDeriveWoAttrs)]
pub fn my_derive_macro_wo_attrs(input: pm::TokenStream) -> pm::TokenStream {
    let input = pm2::TokenStream::from(input);
    let output = proc_macro_template_example_core::my_derive_macro_wo_attrs(input);
    pm::TokenStream::from(output)
}

#[proc_macro_derive(MyDeriveWAttrs, attributes(helper1, helper2))]
pub fn my_derive_macro_w_attrs(input: pm::TokenStream) -> pm::TokenStream {
    let input = pm2::TokenStream::from(input);
    let output = proc_macro_template_example_core::my_derive_macro_w_attrs(input);
    pm::TokenStream::from(output)
}

#[proc_macro_attribute]
pub fn my_attr_macro(attr: pm::TokenStream, item: pm::TokenStream) -> pm::TokenStream {
    let attr = pm2::TokenStream::from(attr);
    let item = pm2::TokenStream::from(item);
    let output = proc_macro_template_example_core::my_attr_macro(attr, item);
    pm::TokenStream::from(output)
}
