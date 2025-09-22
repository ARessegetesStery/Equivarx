use equivarx::{App, WindowDisplay};

fn main() {
    println!("OUT_DIR = {}", std::env::var("OUT_DIR").unwrap());

    App::init()
        .load_modules(vec![Box::new(WindowDisplay::default())])
        .run();
}
