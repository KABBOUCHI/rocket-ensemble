extern crate proc_macro;

use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Rocket)]
pub fn derive_rocket(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;

    quote! {
        impl<'r> rocket::request::FromParam<'r> for #name {
            type Error = &'r str;

            fn from_param(param: &'r str) -> Result<Self, Self::Error> {
                rocket::tokio::task::block_in_place(move || {
                    rocket::tokio::runtime::Handle::current()
                        .block_on(async move { 
                            User::find(param.parse().map_err(|_| "Model Not Found")?).await.map_err(|_| "Model Not Found") 
                        })
                })
            }
        }

        impl<'r> rocket::response::Responder<'r, 'static> for #name {
            fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
                rocket::serde::json::Json(self).respond_to(request)
            }
        }
    }.into()
}
