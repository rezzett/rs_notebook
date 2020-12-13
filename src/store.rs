
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
        self.notes.push(Note {text: text.to_string()})
        // TODO save
    }

    pub fn remove_note(&mut self, index: usize) {
        self.notes.remove(index);
        // TODO save
    }

    pub fn get_notes(&self) -> &[Note] {
        &self.notes
    }

    fn save(&self) {
        unimplemented!();
    }

    fn read(&mut self) {
        unimplemented!();
    }
}