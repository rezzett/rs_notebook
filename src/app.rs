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
                    let input = App::get_input("Enter your note: ");
                    if input.len() < 3 {
                        println!("The note must be at least 3 characters long");
                        continue;
                    }
                    self.store.add_note(input.as_str());
                    self.state = AppState::Menu;
                }
                AppState::Delelte => {
                    if self.check_empty() {
                        println!("There is no notes yet!");
                        self.state = AppState::Menu;
                        continue;
                    }
                    let index = App::get_input("Enter number of note to remove: ");
                    if let Ok(idx) = index.parse::<usize>() {
                        if idx > (self.store.get_notes().len() - 1) {
                            println!("A note with index {} dose not exists!", idx);
                            self.state = AppState::Menu;
                            continue;
                        }
                        self.store.remove_note(idx);
                    } else {
                        println!("Invalid index!");
                        self.state = AppState::Menu;
                    }
                    self.state = AppState::Menu;
                }
                AppState::Show => {
                    if self.check_empty() {
                        self.state = AppState::Menu;
                        continue;
                    }
                    println!("Thre is no notes yet!");
                    println!("///////////// LIST OF NOTES ///////////////\n");
                    for (idx, note) in (self.store.get_notes()).iter().enumerate() {
                        println!("{}.) {}", idx, note);
                    }
                    println!("\n///////////////////////////////////////////");
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

    fn get_input(prompt: &str) -> String {
        println!("{}", prompt);
        let mut buf = String::new();
        std::io::stdin()
            .read_line(&mut buf)
            .expect("[ERROR]: Input failed at function app::App::get_input");
        buf.trim().to_string()
    }

    fn check_empty(&self) -> bool {
        self.store.get_notes().is_empty()
    }
}
