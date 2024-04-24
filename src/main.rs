use speedy2d::Window;

mod body;
mod my_window_handler;

use body::Body;
use crate::my_window_handler::MyWindowHandler;

fn main() {
    let earth = Body::new(100.0, 100.0, 100.0);
    let mars = Body::new(400.0, 400.0, 50.0);

    let bodies: Vec<Body> = vec![earth, mars];

    let window = Window::new_centered("Gravitação Universal", (640, 480)).unwrap();
    let my_window_handler = MyWindowHandler { bodies };
    window.run_loop(my_window_handler);
    println!("Hello, world!");
}
