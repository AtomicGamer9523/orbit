//! Orbit Core

#![no_std]

extern crate alloc;
use alloc::string::String;
use alloc::boxed::Box;
use alloc::vec::Vec;
mod impls;

/// Orbit Event
///
/// An event that can be emitted and handled by the OrbitBus
///
/// # Example
///
/// ```rust
/// # use orbit::*;
/// #[derive(OrbitEvent)]
/// struct MessageEvent {
///     message: String,
///     author: String,
/// }
/// 
/// fn on_message(event: &MessageEvent) {
///     println!("{}: {}", event.author, event.message);
/// }
///
/// #[bus(events = [MessageEvent])]
/// struct Bus;
///
/// let mut bus = Bus::init();
/// bus.on(on_message);
/// bus.emit(MessageEvent {
///     message: "Hello, World!".to_string(),
///     author: "Anonymous".to_string(),
/// });
/// ```
pub trait OrbitEvent {
    fn name(&self) -> &'static str;
}

pub trait OrbitEventHandler<E>
where
    E: OrbitEvent,
{
    fn handle(&self, event: &E);
}

pub type Handlers<E> = Array<Heap<dyn OrbitEventHandler<E>>>;
pub type Array<T> = Vec<T>;
pub type Heap<T> = Box<T>;

pub trait OrbitBusEventHandler<E: OrbitEvent> {
    fn on<H>(&mut self, handler: H) where
        H: OrbitEventHandler<E> + 'static;
    fn emit(&self, event: E);
}

pub trait OrbitBus {
    fn init() -> Self;
}
