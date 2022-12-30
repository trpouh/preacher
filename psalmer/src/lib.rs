extern crate serde;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::Parser;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn psalm_context(_: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(input as ItemStruct);

    // println!("args: {:#?}", item_struct);

    //TODO: parametrize PsalmInfo

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        let info = syn::Field::parse_named
            .parse2(quote! {
                #[serde(flatten)]
                pub info: PsalmInfo
            })
            .unwrap();

        let vars = syn::Field::parse_named
            .parse2(quote! {
                #[serde(skip_deserializing)]
                pub vars: Option<HashMap<String,String>>
            })
            .unwrap();

        fields.named.push(info);
        fields.named.push(vars);
    }

    return quote! { #item_struct }.into();
}
