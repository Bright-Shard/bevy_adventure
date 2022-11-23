use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Event)]
pub fn derive_event(target: TokenStream) -> TokenStream {
    // The struct we're modifying into an Event
    let component: syn::ItemStruct = syn::parse(target)
        .expect("Failed to parse target of derive(Event)");
    // The name of the struct
    let name = component.ident;

    // Modify the struct & return it
    quote!{
        impl crate::events::Event for #name {
            fn new(handler: std::sync::Arc<std::sync::Mutex<dyn crate::events::EventHandler>>) -> Self {
                Self(handler)
            }
        }
    }.into()
}
