
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

pub struct Store {
    notes: Vec<Note>
}

impl Store {
    pub fn new() -> Store {
        Store {notes: vec![]}
    }

    pub fn add_note(&mut self, text: &str) {
        unimplemented!();
    }

    pub fn remove_note(&mut self, index: usize) {
        unimplemented!();
    }

    pub fn get_notes(&self) -> Vec<Note> {
        unimplemented!();
    }

    fn save(&self) {
        unimplemented!();
    }

    fn read(&mut self) {
        unimplemented!();
    }
}