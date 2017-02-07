//! The `data` module holds all the various structs and traits related to
//! representing the dialogue tree.
extern crate uuid;
use self::uuid::Uuid;

/// Conversation Participant, someone who participates in a conversation.
#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Debug)]
pub struct Participant {
    id: Uuid,
    name: String,
}

impl Participant {
    pub fn new(name: String) -> Self {
        let rand_id = Uuid::new_v4(); // ensures the use of a unique identifier for the participant
        Participant{ id: rand_id, name: name }
    }
}

/// A line is like a line in a play. A piece of text that someone reads out-loud.
/// For this reason, a line has an associated participant and some text.
// TODO: figure out what to do if multiple people speak the same line at the same time.
pub struct Line {
    participant: Participant,
    message: String,
}

/// A conversation is the actual dialogue that occurs between people
pub struct Conversation {
    participants: Vec<Participant>,
    dialogue: Vec<Line>,
}

/// The dialogue tree itself is just a vector of conversations, these conversations form the branches
/// of the tree.
pub struct DiaTree {
    conversations: Vec<Conversation>,
}
