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
