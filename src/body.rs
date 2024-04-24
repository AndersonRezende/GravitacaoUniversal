use speedy2d::color::Color;
use speedy2d::Graphics2D;

pub struct Body {
    pub x: f32,
    pub y: f32,
    pub m: f32,
}

impl Body {
    pub fn new(x: f32, y: f32, m: f32) -> Body {
        Body { x, y, m }
    }

    pub fn draw(&self, graphics2d: &mut Graphics2D) {
        graphics2d.draw_circle((self.x, self.y), 75.0, Color::BLUE);
    }
}