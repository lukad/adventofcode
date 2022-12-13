use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[derive(FromDeriveInput)]
#[darling(attributes(date))]
struct DateAttributes {
    year: usize,
    day: usize,
}

#[proc_macro_derive(Date, attributes(date))]
pub fn derive_date_solution(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let DateAttributes { year, day } = match FromDeriveInput::from_derive_input(&input) {
        Ok(v) => v,
        Err(e) => return e.write_errors().into(),
    };

    let ident = input.ident;

    let result = quote! {
        inventory::submit!(&#ident as &dyn Solution);

        impl Date for #ident {
            fn year(&self) -> usize {
                #year
            }

            fn day(&self) -> usize {
                #day
            }
        }
    };

    result.into()
}
