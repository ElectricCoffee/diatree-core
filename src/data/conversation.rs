extern crate uuid;
use self::uuid::Uuid;

use super::{Participant, Line};

/// A conversation is the actual dialogue that occurs between people
pub struct Conversation {
    id: Uuid,
    participants: Vec<Participant>,
    dialogue: Vec<Line>,
}

impl Conversation {
    pub fn new(parts: Vec<Participant>, lines: Vec<Line>) -> Self {
        // TODO: add a check to see if the participants are the same as the ones saying the lines
        Conversation {
            id: Uuid::new_v4(),
            participants: parts,
            dialogue: lines,
        }
    }

    pub fn new_single_participant(part: Participant, lines: Vec<Line>) -> Self {
        Conversation::new(vec![part], lines)
    }
}
