extern crate uuid;
use self::uuid::Uuid;

use super::{Participant, Line};

/// A conversation is the actual dialogue that occurs between people
pub struct Conversation {
    id: Uuid,
    participants: Vec<Participant>,
    dialogue: Vec<Line>,
}
