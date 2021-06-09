use chrono::{DateTime, Datelike, FixedOffset, Local, NaiveDateTime, TimeZone, Timelike, Utc};
use std::fmt;
const STORE_NAME: &'static str = "store.db";

#[derive(Debug)]
pub struct Note {
    pub text: String,
    datetime: DateTime<Utc>,
}

impl Note {
    pub fn new(text: &str, datetime: DateTime<Utc>) -> Note {
        Note {
            text: text.to_string(),
            datetime,
        }
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}\t{}",
            self.text,
            self.datetime.format("%d.%m.%Y %H:%M:%S %z")
        )
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
            let mut data = line.split('\t');
            let text = data.next().unwrap();
            let time = data.next().unwrap();
            let time = DateTime::parse_from_str(time, "%d.%m.%Y %H:%M:%S %z").expect("FAILED!!!!");
            notes.push(Note::new(text, Utc.from_utc_datetime(&time.naive_utc())));
        }
        Store { notes }
    }

    pub fn add_note(&mut self, text: &str) {
        self.notes.push(Note::new(text, Utc::now()));
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
        let mut contents = String::new();
        for note in &self.notes {
            contents.push_str(&format!("{}\n", note));
        }
        std::fs::write(STORE_NAME, contents)
            .expect("[ERROR]: Failed to save data at store::Store::drop");
    }
}
