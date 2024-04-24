use speedy2d::color::Color;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::Graphics2D;
use crate::body::Body;

pub struct MyWindowHandler {
    pub bodies: Vec<Body>
}

impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper<()>, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::from_rgb(0.8, 0.9, 1.0));
        //graphics.draw_circle((100.0, 100.0), 75.0, Color::BLUE);

        for body in &self.bodies {
            body.draw(graphics);
        }

        // Request that we draw another frame once this one has finished
        helper.request_redraw();
    }
}