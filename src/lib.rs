#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, Data, DataEnum, DeriveInput, Ident, Variant};

#[proc_macro_derive(ToResponse, attributes(code))]
pub fn to_http_error_code(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);

    let name = &ast.ident;
    let Data::Enum(enum_data) = ast.data else {
        panic!("only avaliable for enum");
    };
    impl_into_response(name, enum_data).into()
}

fn impl_into_response(_name: &Ident, enum_data: DataEnum) -> proc_macro2::TokenStream {
    let _variants: Vec<proc_macro2::TokenStream> = enum_data
        .variants
        .iter()
        .map(|v| make_enum_variant(v))
        .collect();
    cfg_if::cfg_if! {
        if #[cfg(all(feature = "axum", not(feature = "rocket")))] {
            quote! {
                impl axum::response::IntoResponse for #name {
                    fn into_response(self) -> axum::response::Response {
                        match &self {
                            #(Self::#variants,)*
                        }
                    }
                }
            }
        } else if #[cfg(all(feature = "rocket", not(feature = "axum")))] {
            quote! {
                impl<'r, 'o: 'r> ::rocket::response::Responder<'r, 'o> for #_name {
                    fn respond_to(self, request: &'r rocket::request::Request<'_>) -> rocket::response::Result<'o> {
                        match &self {
                            #(Self::#_variants,)*
                        }
                    }
                }
            }
        } else {
            unimplemented!("Use rocket OR axum feature!");
        }
    }
}

fn make_enum_variant(variant: &Variant) -> proc_macro2::TokenStream {
    let _ident = &variant.ident;
    let _fields = match &variant.fields {
        syn::Fields::Unit => quote!(),
        syn::Fields::Named(_) => quote!({ .. }),
        syn::Fields::Unnamed(fields) => {
            let unnamed = fields
                .unnamed
                .iter()
                .map(|_| quote!(_))
                .collect::<Vec<proc_macro2::TokenStream>>();
            quote!((#(#unnamed),*))
        }
    };
    let attrs: Vec<&Attribute> = variant
        .attrs
        .iter()
        .filter(|attr| attr.path.is_ident("code"))
        .collect();

    let code = if let Some(attr) = attrs.get(0) {
        attr.tokens.clone().to_string()
    } else {
        quote! {(500)}.to_string()
    };
    //Trimming ( )
    let _code: proc_macro2::TokenStream = code[1..code.len() - 1].parse().unwrap();
    cfg_if::cfg_if! {
        if #[cfg(all(feature = "axum", not(feature = "rocket")))] {
             quote! { #_ident #_fields => axum::http::StatusCode::from_u16(#_code).unwrap().into_response()}
         } else if #[cfg(all(feature = "rocket", not(feature = "axum")))] {
             quote! { #_ident #_fields =>
             {}.respond_to(request).map(|mut resp| {
                     resp.set_status(rocket::http::Status::from_code(#_code).unwrap());
                     resp
                 })

             }
         } else {
             unreachable!("Use rocket OR axum feature!");
         }
    }
}
