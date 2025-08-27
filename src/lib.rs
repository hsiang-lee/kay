//! Kay is an experimental high-performance distributed actor system framework for Rust.
//! It is developed as a component for [Citybound](https://cityboundsim.com)
//! (a city simulation game) but aims to be general-purpose.
//!
//! Kay is inspired by Erlang/OTP and similar actor-oriented approaches,
//! since it allows you to **build a distributed application from isolated actors
//! that communicate only through message-passing,** which works transparently across
//! processor and even network boundaries.
//!
//! The main abstractions are [Classes](TODO) of actors that live inside an [Actor System](TODO),
//! adressed by [TypedID](TODO)s. [Classes](TODO) can implement [Traits](TODO), allowing generic dynamic dispatch.
//!
//! Kay lacks many higher level features and error-handling mechanisms that other actor system frameworks offer
//! since it puts a focus on high-performance and memory efficiency. This is achieved
//! by storing actor state and message queues in consecutive chunks of memory,
//! inspired by the data-oriented game engine design philosophy.
//! The [Compact](https://TODO) library is used to help with this, offering
//! serialisation-free linear memory layouts for plain old data and nested datastructures.
//! This does, in turn, impose the constraint that actor state and messages need to implement
//! [Compact](https://TODO)

#![warn(missing_docs)]
extern crate chunky;
extern crate compact;
#[macro_use]
extern crate compact_macros;
extern crate byteorder;
extern crate core;
#[cfg(feature = "browser")]
#[macro_use]
extern crate stdweb;
#[cfg(feature = "server")]
extern crate tungstenite;
extern crate url;
#[cfg(feature = "serde-serialization")]
#[macro_use]
extern crate serde_derive;
#[cfg(feature = "serde-serialization")]
extern crate serde;

macro_rules! make_array {
    ($n:expr, $element:expr) => {{
        use std::mem::MaybeUninit;

        let mut items: [MaybeUninit<_>; $n] = unsafe { MaybeUninit::uninit().assume_init() };

        for i in 0..$n {
            items[i] = MaybeUninit::new($element(i));
        }

        unsafe {
            let result = std::ptr::read(&items as *const _ as *const [_; $n]);
            std::mem::forget(items);
            result
        }
    }};
}

mod actor;
mod actor_system;
mod class;
mod external;
mod id;
mod messaging;
mod networking;
mod storage_aware;
mod tuning;
mod type_registry;

pub use self::actor::{Actor, ActorOrActorTrait, TraitIDFrom};
pub use self::actor_system::{ActorSystem, World};
pub use self::external::External;
pub use self::id::{MachineID, RawID, TypedID};
pub use self::messaging::{Fate, Message, Packet};
pub use self::networking::Networking;
pub use self::tuning::Tuning;
