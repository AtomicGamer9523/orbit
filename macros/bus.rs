use proc_macro2::TokenStream as T;

/// #[bus(events = [Message, Player])]
pub fn bus(config: T, input: T) -> T {
    input
}