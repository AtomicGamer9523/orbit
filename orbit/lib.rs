//! Orbit Event System
//!
//! Focused on simplicity and performance.
//!
//! # Example using the `#[bus]` macro
//!
//! ```rust
//! # use orbit::*;
//! #[bus(Message, Player)]
//! struct Bus;
//! 
//! struct Player(String);
//!
//! struct Message {
//!     id: u64,
//!     text: String,
//!     author: String,
//! }
//! 
//! struct MessageHandler;
//!
//! impl OrbitEventHandler<Message> for MessageHandler {
//!     fn handle(&self, msg: &Message) {
//!         println!("'{}' by '{}' (#{})", msg.text, msg.author, msg.id);
//!     }
//! }
//!
//! fn on_message(message: &Message) {
//!     println!("{}: {}", message.author, message.text);
//! }
//!
//! # fn main() {
//! let mut bus = Bus::init();
//! bus.sub(|player: &Player| {
//!     println!("Player joined: {}", player.0);
//! });
//! bus.sub(MessageHandler);
//! bus.sub(on_message);
//!
//! bus.emit(Message {
//!     id: 1,
//!     text: "Hello, world!".to_string(),
//!     author: "Alice".to_string()
//! });
//! bus.emit(Player("Bob".to_string()));
//! # }
//! ```
//!
//! For the architechture of the system, see the [here]
//!
//! [here]: crate::__docs__::architechture

#![no_std]

pub use orbitcore::*;

#[cfg(feature = "__m")]
pub use orbitmacros::*;

#[cfg(feature = "__m")]
__import_internals__!();
