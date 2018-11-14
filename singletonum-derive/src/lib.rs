#[macro_use]
extern crate quote;
extern crate syn;
extern crate proc_macro;
#[macro_use]
extern crate failure;
extern crate heck;

use failure::Error;
use syn::Ident;
use proc_macro::TokenStream;
use quote::quote;
use heck::ShoutySnakeCase;

#[proc_macro_derive(Singleton)]
pub fn singleton_derive(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = impl_singleton(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}

fn impl_singleton(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let static_var_name = format!("{}_SINGLETON", name.as_ref().to_shouty_snake_case());
    let static_var_ident = Ident::from(static_var_name);
    quote! {
        static #static_var_ident: singletonum::OnceCell<#name> = singletonum::OnceCell::INIT;

        impl singletonum::Singleton for #name {
            fn get_instance(init: &Self::Init) -> &'static Self {
                #static_var_ident.get_or_init(|| Self::init(init))
            }
        }
    }
}
