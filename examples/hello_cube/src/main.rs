use equivarx::{App, WindowDisplay};

fn main() {
    App::init()
        .load_modules(vec![Box::new(WindowDisplay::default())])
        .run();
}
