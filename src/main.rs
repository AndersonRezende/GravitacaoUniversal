use speedy2d::color::Color;
use speedy2d::Window;

mod body;
mod my_window_handler;
mod universal_gravitation;
mod cartesian_utils;

use body::Body;
use crate::my_window_handler::MyWindowHandler;

fn main() {
    let earth = Body::new(100.0, 100.0, 100.0, 0.0, 75.0, 1.0, 1.0, Color::BLUE);
    let mars = Body::new(950.0, 500.0, 50.0, 0.0, 60.0, 0.0, 0.0, Color::RED);

    let bodies: Vec<Body> = vec![earth, mars];

    let window = Window::new_centered("Gravitação Universal", (1900, 1000)).unwrap();
    let my_window_handler = MyWindowHandler { bodies };
    window.run_loop(my_window_handler);
}
