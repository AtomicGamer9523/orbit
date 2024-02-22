//! Core of the Orbit event system.
//!
//! This crate contains only the traits, literraly nothing else.

#![no_std]

/// This trait is used to handle events in the Orbit system.
///
/// This is a trait that all event handlers must implement.
/// Note that this trait is automatically implemented for any function that
/// takes a `&Event` and returns void.
/// 
/// # Example
///
/// ```rust
/// # use orbit::*;
/// # struct Bus;
/// # impl OrbitBus for Bus {
/// #     fn init() -> Self { todo!() }
/// # }
/// # struct MessageEvent {
/// #     id: u64,
/// #     message: String,
/// #     author: String,
/// # }
/// # impl OrbitBusManager<Message> for Bus {
/// #     fn sub<H>(&mut self, handler: H) where
/// #         H: OrbitEventHandler<Message> + 'static
/// #     { todo!() }
/// #     fn emit(&self, event: Message) { todo!() }
/// # }
/// struct MessageEventHandler;
///
/// impl OrbitEventHandler<MessageEvent> for MessageEventHandler {
///     fn handle(&self, event: &MessageEvent) {
///         println!("Received message: {}", event.id);
///     }
/// }
///
/// fn on_message(event: &MessageEvent) {
///     println!("{}: {}", event.author, event.message);
/// }
///
/// # fn main() {
/// let mut bus = Bus::init();
/// bus.on(MessageEventHandler);
/// bus.on(on_message);
/// # }
/// ```
pub trait OrbitEventHandler<Event> {
    /// Function that gets invoked when the event is emitted.
    ///
    /// # Notice
    ///
    /// This function should not block the thread, as it will block the entire
    /// event loop. It is recommended to spawn a new thread or use async/await
    /// to handle the event.
    fn handle(&self, event: &Event);
}

/// This trait is used to manage the event bus in the Orbit system.
///
/// This is a trait that must be implemented by the event bus,
/// for it to properly subscribe and emit events.
///
/// It will get automatically implemented if you use the `#[bus]` macro.
///
/// # Example using the `#[bus]` macro
///
/// ```rust
/// # use orbit::*;
/// #[bus(Message, Player)]
/// struct Bus;
/// ```
///
/// # Example without the `#[bus]` macro
///
/// ```rust
/// # use orbit::*;
/// struct Bus {
///     // Code to manage the event handlers goes here
/// }
/// struct Message(String);
/// impl OrbitBusManager<Message> for Bus {
///     fn sub<H>(&mut self, handler: H) where
///         H: OrbitEventHandler<Message> + 'static
///     {
///         // Code to subscribe the handler to the event bus goes here
///     }
///     fn emit(&self, event: Message) {
///         // Code to emit the event goes here
///     }
/// }
/// ```
pub trait OrbitBusManager<Event> {
    /// Subscribes an event handler to the event bus.
    ///
    /// For examples see the [`OrbitBus`] trait.
    /// 
    /// [`OrbitBus`]: crate::OrbitBus
    fn sub<H>(&mut self, handler: H) where
        H: OrbitEventHandler<Event> + 'static;
    /// Emits an event to the event bus.
    ///
    /// For examples see the [`OrbitBus`] trait.
    ///
    /// [`OrbitBus`]: crate::OrbitBus
    fn emit(&self, event: Event);
}

/// An event bus that can be used to manage events in the Orbit system.
/// 
/// This is the central hub for all events in the system.
/// It is used to subscribe and emit events.
/// (see the disclaimer [here](#disclaimer))
///
/// # Example
///
/// ```rust
/// # use orbit::*;
/// #[bus(Message, Player)]
/// struct Bus;
/// 
/// struct Player(String);
///
/// struct Message {
///     id: u64,
///     text: String,
///     author: String,
/// }
/// 
/// struct MessageHandler;
///
/// impl OrbitEventHandler<Message> for MessageHandler {
///     fn handle(&self, msg: &Message) {
///         println!("'{}' by '{}' (#{})", msg.text, msg.author, msg.id);
///     }
/// }
///
/// fn on_message(message: &Message) {
///     println!("{}: {}", message.author, message.text);
/// }
///
/// # fn main() {
/// let mut bus = Bus::init();
/// bus.sub(|player: &Player| {
///     println!("Player joined: {}", player.0);
/// });
/// bus.sub(MessageHandler);
/// bus.sub(on_message);
///
/// bus.emit(Message {
///     id: 1,
///     text: "Hello, world!".to_string(),
///     author: "Alice".to_string()
/// });
/// bus.emit(Player("Bob".to_string()));
/// # }
/// ```
///
/// # Disclaimer
///
/// Actually, I lied. For subscribing and emitting events, you should use the
/// [`OrbitBusManager`] trait. This trait is in reality only used to initialize the event bus.
/// But the `OrbitBusManager` trait gets automatically implemented by the `#[bus]` macro,
/// so you don't have to worry about it.
/// Unless of course, you want to implement it yourself raw.
/// In that case, you should avoid the `#[bus]` macro
/// at all costs, as it forbids you from adding anything
/// to the `Bus` struct, which is required for manual implementation.
pub trait OrbitBus {
    /// Initializes the event bus.
    fn init() -> Self;
}

impl<F, Event> OrbitEventHandler<Event> for F
where
    F: Fn(&Event),
{
    /// Automatically implement [`OrbitEventHandler`]
    /// for any function that takes an `Event` and returns `()`.
    ///
    /// [`OrbitEventHandler`]: crate::OrbitEventHandler
    #[inline]
    fn handle(&self, event: &Event) {
        self(event);
    }
}
