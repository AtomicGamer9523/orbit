//! Macros for Orbit

use proc_macro::TokenStream;

mod bus;
mod event;

#[inline]
#[allow(non_snake_case)]
#[proc_macro_derive(OrbitEvent)]
pub fn OrbitEvent(input: TokenStream) -> TokenStream {
    event::orbit_event(input.into()).into()
}

#[inline]
#[proc_macro_attribute]
pub fn bus(config: TokenStream, input: TokenStream) -> TokenStream {
    bus::bus(config.into(), input.into()).into()
}
