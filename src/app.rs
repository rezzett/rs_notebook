use crate::store::*;

enum AppState {
    Menu,
    Add,
    Delelte,
    Show,
    Quit,
}

pub struct App {
    state: AppState,
    store: Store,
}

impl App {
    pub fn new() -> App {
        App {
            state: AppState::Menu,
            store: Store::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            match self.state {
                AppState::Menu => {
                    App::menu();
                    let user_input = App::get_input("Choose command:");
                    self.command_handler(user_input.as_str());
                }
                AppState::Add => {
                    println!("ADD");
                    self.add();
                    self.state = AppState::Menu;
                }
                AppState::Delelte => {
                    self.del();
                    println!("DEL");
                    self.state = AppState::Menu;
                }
                AppState::Show => {
                    self.show();
                    println!("SHOW");
                    self.state = AppState::Menu;
                }
                AppState::Quit => {
                    break;
                }
            }
        }
    }

    fn menu() {
        println!("===== MENU =====");
        println!("1. (A)dd task");
        println!("2. (S)how tasks");
        println!("3. (D)elete task");
        println!("4. (Q)uit");
        println!("================");
    }

    fn command_handler(&mut self, user_input: &str) {
        match user_input {
            "a" | "A" | "1" => self.state = AppState::Add,
            "d" | "D" | "3" => self.state = AppState::Delelte,
            "s" | "S" | "2" => self.state = AppState::Show,
            "q" | "Q" | "4" => self.state = AppState::Quit,
            _ => {
                println!("Invalid command!");
                self.state = AppState::Menu;
            }
        }
    }

    fn add(&mut self) {
        let input = App::get_input("Enter your note: ");
        self.store.add_note(input.as_str());
    }

    fn del(&mut self) {
        let index = App::get_input("Enter number of note to remove: ");
        if let Ok(idx) = index.parse::<usize>() {
            self.store.remove_note(idx);
        } else {
            println!("Invalid index!");
            self.state = AppState::Menu;
        }
    }

    fn show(&self) {
        for note in self.store.get_notes() {
            println!("{}", note.text);
        }
    }

    fn get_input(prompt: &str) -> String {
        println!("{}", prompt);
        let mut buf = String::new();
        std::io::stdin()
            .read_line(&mut buf)
            .expect("[ERROR]: Input failed at function app::App::get_input");
        buf.trim().to_string()
    }
}
