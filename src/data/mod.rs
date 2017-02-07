//! The `data` module holds all the various structs and traits related to
//! representing the dialogue tree.
pub use self::participant::Participant;
mod participant;

pub use self::line::Line;
mod line;

pub use self::conversation::Conversation;
mod conversation;

pub use self::diatree::DiaTree;
mod diatree;
