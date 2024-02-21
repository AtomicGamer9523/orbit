#![allow(missing_docs)]

use orbit::*;

#[bus(events = [Message, Player])]
// struct Bus;

struct Bus {
    message_handlers: Handlers<Message>,
    player_handlers: Handlers<Player>,
}

impl OrbitBus for Bus {
    #[inline]
    fn init() -> Self {
        Self {
            message_handlers: Array::new(),
            player_handlers: Array::new(),
        }
    }
}

#[derive(OrbitEvent, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Player {
    name: String
}

// impl OrbitEvent for Player {
//     #[inline(always)]
//     fn name(&self) -> &'static str { "Player" }
// }
// 
// impl OrbitBus<Player> for Bus {
//     fn on<H>(&mut self, handler: H) where
//         H: OrbitEventHandler<Player> + 'static
//     {
//         self.player_handlers.push(Box::new(handler));
//     }
//     fn emit(&self, event: Player) {
//         for handler in &self.player_handlers {
//             handler.handle(&event);
//         }
//     }
// }

#[derive(OrbitEvent, Debug, Clone, Eq)]
// #[derive(Debug, Clone, Eq, Ord)]
struct Message {
    id: u64,
    text: String,
    author: String,
}
// impl OrbitEvent for Message {
//     #[inline(always)]
//     fn name(&self) -> &'static str { "Message" }
// }
// impl OrbitBus<Message> for Bus {
//     fn on<H>(&mut self, handler: H) where
//         H: OrbitEventHandler<Message> + 'static
//     {
//         self.message_handlers.push(Box::new(handler));
//     }
//     fn emit(&self, event: Message) {
//         for handler in &self.message_handlers {
//             handler.handle(&event);
//         }
//     }
// }
impl PartialEq for Message {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
// impl PartialOrd for Message {
//     #[inline]
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         self.id.partial_cmp(&other.id)
//     }
// }
// impl ::core::hash::Hash for Message {
//     #[inline]
//     fn hash<H>(&self, state: &mut H)
//     where
//         H: ::core::hash::Hasher,
//     {
//         self.id.hash(state);
//     }
// }

fn on_message(message: &Message) {
    println!("Received message: {:?}", message);
}

fn main() {
    let mut bus = Bus::init();
    bus.on(on_message);
    bus.on(on_player_joined);

    bus.emit(Message {
        id: 1,
        text: "Hello, world!".to_string(),
        author: "Alice".to_string()
    });
    bus.emit(Player {
        name: "Bob".to_string()
    });
}

fn on_player_joined(player: &Player) {
    println!("Player joined: {}", player.name);
}
