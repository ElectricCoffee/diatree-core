use super::{Participant, Line};

/// A conversation is the actual dialogue that occurs between people
pub struct Conversation {
    participants: Vec<Participant>,
    dialogue: Vec<Line>,
}
