use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

use crate::{menu::expand_menu, selectable::expand_select_value};

mod menu;
mod selectable;

#[proc_macro_derive(SelectValue, attributes(display_as))]
pub fn derive_select_value(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    expand_select_value(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_derive(Menu, attributes(menu))]
pub fn derive_menu(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    expand_menu(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
