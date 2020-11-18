
#[derive(Debug)]
pub struct Note {
    text: String
    // TODO add time
}

impl Note {
    pub fn new(text: &str) -> Note {
        Note {text: text.to_string()}
    }
}