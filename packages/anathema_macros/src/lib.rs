use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(BBComponent, attributes(bb_component))]
pub fn derive_bb_component(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    let mut state_type: Option<syn::Type> = None;

    for attr in &input.attrs {
        if attr.path().is_ident("bb_component") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("state") {
                    let value = meta.value()?;
                    state_type = Some(value.parse()?);
                }
                Ok(())
            })
            .expect("meowmeowmew");
        }
    }

    let state_init = match &state_type {
        Some(state_type) => quote! { || #state_type::new() },
        None => quote! { || () },
    };

    let expanded = quote! {
        use anathema::prelude::*;

        impl crate::BBComponent for #struct_name {
            fn ident() -> &'static str {
                stringify!(#struct_name)
            }
            fn register_to(builder: &mut anathema::runtime::Builder<()>) -> Result<(), anathema::runtime::Error> {

                builder.prototype(
                    Self::ident(),
                    Self::load_template().to_template(),
                    || Self,
                    #state_init,
                )
            }
            fn load_template() -> &'static str {
                include_str!("./template.aml")
            }
        }
    };

    TokenStream::from(expanded)
}
