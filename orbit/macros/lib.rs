//! Macros for Orbit

use proc_macro2::TokenStream as T;
use proc_macro::TokenStream;

mod util;
pub(crate) use util::*;

#[cfg_attr(feature = "v1", path = "v1.rs")]
#[cfg_attr(feature = "v2", path = "v2.rs")]
mod _impl;

/// A macro to automatically generate the necessary code to manage an event bus.
///
/// # Example
///
/// ```rust
/// # use orbit::*;
/// #[bus(Message)]
/// struct Bus;
///
/// struct Message {
///     id: u64,
///     text: String,
///     author: String,
/// }
///
/// fn on_message(message: &Message) {
///     println!("Received message: {:?}", message);
/// }
///
/// # fn main() {
/// let mut bus = Bus::init();
/// bus.on(on_message);
/// bus.emit(Message {
///     id: 0,
///     text: "Hello, World!".to_string(),
///     author: "Alice".to_string()
/// });
/// # }
/// ```
#[proc_macro_attribute]
pub fn bus(config: TokenStream, input: TokenStream) -> TokenStream {
    let config: T = config.into();
    let input: T = input.into();
    let res: T = match _impl::bus_impl(config, input) {
        Err(e) => e.to_compile_error(),
        Ok(t) => t
    };
    res.into()
}

/// # !!!INTERNAL!!! DO NOT USE!
///
/// This macro is used internally by Orbit to import the necessary items
/// for the macros to properly generate code.
#[proc_macro]
#[doc(hidden)]
#[inline(always)]
pub fn __import_internals__(_: TokenStream) -> TokenStream {
    _impl::import_internals().into()
}
