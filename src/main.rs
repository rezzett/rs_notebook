use app::App;


mod app;
mod store;

fn main() {
    let mut app = App::new();
    app.run();
}
