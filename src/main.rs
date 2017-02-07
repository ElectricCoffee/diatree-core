mod data;

fn main() {
}


#[cfg(test)]
mod test {
    use super::data::*;
    #[test]
    fn ensure_different() {
        // The participants should be different despite being called with the same parameter
        // because of how UUIDs work
        let name = "Johnson".to_string();
        let p1 = Participant::new(name.clone());
        let p2 = Participant::new(name);
        assert_ne!(p1, p2);
    }
}
