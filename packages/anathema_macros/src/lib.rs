use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(BBComponent)]
pub fn derive_bb_component(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    let expanded = quote! {
        impl crate::BBComponent for #struct_name {
            fn ident() -> &'static str {
                stringify!(#struct_name)
            }
            fn register_to(builder: &mut Builder<()>) -> Result<(), Error> {
                builder.prototype(
                    Self::ident(),
                    Self::load_template().to_template(),
                    || Self,
                    || (),
                )
            }
            fn load_template() -> &'static str {
                include_str!("./template.aml")
            }
        }
    };

    TokenStream::from(expanded)
}
