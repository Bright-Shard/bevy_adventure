use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{self, Field};

#[proc_macro_derive(Event)]
pub fn derive_event(target: TokenStream) -> TokenStream {
    println!("Deriving Event");
    let component: syn::ItemStruct = syn::parse(target)
        .expect("Failed to parse target of derive(Event)");

    if let syn::Fields::Unnamed(fields) = component.fields {
        /*
        fields.unnamed.push(
            Field::parse_unnamed(
                syn::parse(
                    quote! {
                        impl bevy::ecs::schedule::IntoSystemDescriptor
                    }.into()
                ).unwrap()
            ).unwrap()
        )
        */
    }
    
    TokenStream::new()
}
