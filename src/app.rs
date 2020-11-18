enum AppState {
    Menu,
    Add,
    Delelte,
    Show,
    Quit,
}

pub struct App {
    state: AppState,
}

impl App {
    pub fn new() -> App {
        App {
            state: AppState::Menu,
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
                    self.state = AppState::Menu;
                }
                AppState::Delelte => {
                    println!("DEL");
                    self.state = AppState::Menu;
                }
                AppState::Show => {
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

    fn get_input(prompt: &str) -> String {
        println!("{}", prompt);
        let mut buf = String::new();
        std::io::stdin()
            .read_line(&mut buf)
            .expect("[ERROR]: Input failed at function app::App::get_input");
        buf.trim().to_string()
    }
}
