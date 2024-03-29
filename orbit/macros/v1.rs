use crate::*;

pub(crate) fn bus_impl(config: T, input: T) -> syn::Result<T> {
    let events = parse_events(config)?;
    let name = parse_name(input)?;
    let mut handlers = Vec::new();
    let mut impls = Vec::new();
    let mut constructors = Vec::new();
    for event in events {
        let ev = &event.name;
        // let event_type = &event.ty;
        let handler_name = format!("{ev}_handlers").to_lowercase();
        let handler_name = syn::Ident::new(&handler_name, event.span());
        let doc_comment = format!(" Handlers for the `{ev}` event.");
        let doc_comment = syn::LitStr::new(&doc_comment, event.span());
        handlers.push(quote::quote! {
            #[doc = #doc_comment]
            ///
            /// This was automatically generated by the `#[bus]` macro.
            #[doc(hidden)]
            pub #handler_name: ::orbit::__orbit_internals__::Handlers<#ev>,
        });
        if event.sub {
            impls.push(quote::quote! {
                impl ::orbit::OrbitBusSubscribingManager<#ev> for #name {
                    /// Subscribes an event handler to the event bus.
                    ///
                    /// This was automatically generated by the `#[bus]` macro.
                    fn sub<H>(&mut self, handler: H) where
                        H: ::orbit::OrbitEventHandler<#ev> + 'static
                    {
                        self.#handler_name.push(::orbit::__orbit_internals__::Heap::new(handler));
                    }
                }
            });
        }
        if event.emit {
            impls.push(quote::quote! {
                impl ::orbit::OrbitBusEmitingManager<#ev> for #name {
                    /// Emits an event to the event bus.
                    ///
                    /// This was automatically generated by the `#[bus]` macro.
                    fn emit(&self, event: #ev) {
                        for handler in &self.#handler_name {
                            handler.handle(&event);
                        }
                    }
                }
            });
        }
        constructors.push(quote::quote! {
            #handler_name: ::orbit::__orbit_internals__::Array::new(),
        });
    }
    Ok(quote::quote! {
        struct #name {
            #(#handlers)*
        }
        impl ::orbit::OrbitBus for #name {
            /// Initializes the bus with empty handlers.
            ///
            /// This was automatically generated by the `#[bus]` macro.
            fn init() -> Self {
                Self {
                    #(#constructors)*
                }
            }
        }
        #(#impls)*
    })
}

pub(crate) fn import_internals() -> T {
    quote::quote! {
        #[doc(hidden)]
        extern crate alloc;
        #[doc(hidden)]
        pub mod __orbit_internals__ {
            use ::orbitcore::OrbitEventHandler;
            #[cfg(not(feature = "std"))]
            use ::alloc as __alloc;
            #[cfg(feature = "std")]
            use ::std as __alloc;
            use __alloc::{vec::Vec, boxed::Box};
            #[doc(hidden)]
            pub type Heap<T> = Box<T>;
            #[doc(hidden)]
            pub type Array<T> = Vec<T>;
            #[doc(hidden)]
            pub type Handlers<E> = Array<Heap<dyn OrbitEventHandler<E>>>;
        }
    }
}
