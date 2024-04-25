use speedy2d::color::Color;
use speedy2d::Graphics2D;

pub struct Body {
    pub x: f32,
    pub y: f32,
    pub m: f32,
    pub a: f32,
    pub radius: f32,
    pub color: Color,
}

impl Body {
    pub fn new(x: f32, y: f32, m: f32, a: f32, radius: f32, color: Color) -> Body {
        Body { x, y, m, a, radius, color }
    }

    pub fn draw(&self, graphics2d: &mut Graphics2D) {
        graphics2d.draw_circle((self.x, self.y), self.radius, self.color);
    }

    pub fn update() {

    }

    pub fn distance(body1: &Body, body2: &Body) -> f32 {
        ((body2.x - body1.x).powf(2.0) + (body2.y - body1.y).powf(2.0)).sqrt()
    }
}