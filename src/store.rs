const STORE_NAME: &'static str = "store.db";

#[derive(Debug)]
pub struct Note {
    pub text: String, // TODO add time
}

impl Note {
    pub fn new(text: &str) -> Note {
        Note {
            text: text.to_string(),
        }
    }
}

pub struct Store {
    notes: Vec<Note>,
}

impl Store {
    pub fn new() -> Store {
        if !std::path::Path::new(STORE_NAME).exists() {
            std::fs::File::create(STORE_NAME)
                .expect("[ERROR]: Failde to create 'store.db' at store::Store::new()");
        }
        let contents = std::fs::read_to_string(STORE_NAME)
            .expect("[ERROR]: Failed to read content at store::Store::new()");
        let mut notes = vec![];
        for line in contents.lines() {
            notes.push(Note::new(line));
        }
        Store { notes }
    }

    pub fn add_note(&mut self, text: &str) {
        self.notes.push(Note {
            text: text.to_string(),
        })
    }

    pub fn remove_note(&mut self, index: usize) {
        self.notes.remove(index);
    }

    pub fn get_notes(&self) -> &Vec<Note> {
        &self.notes
    }
}

impl Drop for Store {
    fn drop(&mut self) {
        for note in &self.notes {
            let line = format!("{}\n", note.text);
            std::fs::write(STORE_NAME, line)
                .expect("[ERROR]: Failed to save data at store::Store::drop");
        }
    }
}
