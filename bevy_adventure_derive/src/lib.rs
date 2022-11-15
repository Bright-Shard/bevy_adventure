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
        impl Event for #name {
            fn new<Handler: EventHandler, F: IntoEventHandler<Handler = Handler>>(handler: F) -> Self {
                Self(std::sync::Arc::new(
                        std::sync::Mutex::new(
                            <F as IntoEventHandler>::into_event(handler)
                        )
                    )
                )
            }

            fn get_handler(&self) -> Arc<Mutex<dyn EventHandler>> {
                self.0.clone()
            }
        }
    }.into()
}
