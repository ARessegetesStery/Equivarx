use equivarx::{App, Window};

fn main() {
    App::init()
        .load_modules(vec![Box::new(Window::default())])
        .run();
}
