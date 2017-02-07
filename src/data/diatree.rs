use super::Conversation;

/// The dialogue tree itself is just a vector of conversations, these conversations form the branches
/// of the tree.
pub struct DiaTree {
    conversations: Vec<Conversation>,
}
