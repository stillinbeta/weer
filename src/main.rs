mod app;
use app::App;

fn main() {
    let app = App::new();
    if let Err(err) = app.run() {
        println!("{}", err)
    }
}
