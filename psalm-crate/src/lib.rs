#[macro_use]
extern crate serde;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::Parser;
use syn::{parse, parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn psalm_context(_: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(input as ItemStruct);

    // println!("args: {:#?}", item_struct);

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        let field = syn::Field::parse_named
            .parse2(quote! {
                #[serde(flatten)]
                pub info: Option<PsalmInfo>
            })
            .unwrap();
        /*
        let attr = syn::Attribute::parse_inner.parse2(quote! {
            
        }).unwrap();


        field.attrs = attr; */

        fields.named.push(field);
    }

    return quote! { #item_struct }.into();
}

#[proc_macro_attribute]
pub fn psalm(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(input as ItemStruct);

    let _ = parse_macro_input!(args as parse::Nothing);

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! {
                  //#[serde(flatten, default = None)]
                  pub info: Option<PsalmInfo>
                })
                .unwrap(),
        );
    }

    return quote! {
        #item_struct
    }
    .into();
}
