use speedy2d::color::Color;
use speedy2d::Window;

mod body;
mod my_window_handler;
mod universal_gravitation;
mod cartesian_utils;

use body::Body;
use crate::my_window_handler::MyWindowHandler;

fn main() {
    let sun = Body::new(900.0, 500.0, 1989.0000, 0.0, 139.20, 0.0, 0.0, Color::YELLOW);
    let mercury = Body::new(743.0, 343.0, 0.3, 0.0, 4.8, -30.0, 0.0, Color::GRAY);
    let venus = Body::new(692.0, 292.0, 4.8, 0.0, 12.1, -30.0, 0.0, Color::from_rgb(1.0, 0.6, 0.0));
    let earth = Body::new(1149.0, 749.0, 5.9, 0.0, 12.7, 30.0, 0.0, Color::BLUE);
    let mars = Body::new(1297.0, 827.0, 0.6, 0.0, 6.7, 30.0, 1.3, Color::RED);

    let bodies: Vec<Body> = vec![sun, mercury, venus, earth, mars];

    let window = Window::new_centered("Gravitação Universal", (1900, 1000)).unwrap();
    let my_window_handler = MyWindowHandler { bodies };
    window.run_loop(my_window_handler);
}
