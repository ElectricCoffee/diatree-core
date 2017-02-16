use super::Participant;

/// A line is like a line in a play. A piece of text that someone reads out-loud.
/// For this reason, a line has an associated participant and some text.
// TODO: figure out what to do if multiple people speak the same line at the same time.
pub struct Line {
    participant: Participant,
    message: String,
}

impl Line {
    pub fn new(part: Participant, msg: String) -> Self {
        Line { participant: part, message: msg }
    }
}
