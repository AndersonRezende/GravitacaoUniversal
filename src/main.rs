use speedy2d::Window;

use crate::my_window_handler::MyWindowHandler;
use crate::solar_system::solar_system::create_first_five;

mod body;
mod my_window_handler;
mod universal_gravitation;
mod cartesian_utils;
mod solar_system;

fn main() {
    let window = Window::new_centered("Gravitação Universal", (1900, 1000)).unwrap();
    let my_window_handler = MyWindowHandler { bodies: create_first_five() };
    window.run_loop(my_window_handler);
}
