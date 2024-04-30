use speedy2d::color::Color;
use speedy2d::Graphics2D;

#[derive(Debug)]
pub struct Body {
    pub x: f32,
    pub y: f32,
    pub m: f32,
    pub a: f32,
    pub radius: f32,
    pub color: Color,

    pub x_speed: f32,
    pub y_speed: f32,
}

impl Body {
    pub fn new(x: f32, y: f32, m: f32, a: f32, radius: f32, x_speed: f32, y_speed: f32, color: Color) -> Body {
        Body { x, y, m, a, radius, color, x_speed: x_speed, y_speed: y_speed }
    }

    pub fn draw(&self, graphics2d: &mut Graphics2D) {
        graphics2d.draw_circle((self.x, self.y), self.radius, self.color);
    }

    pub fn update(&mut self) {
        self.x += self.x_speed;
        self.y += self.y_speed;
    }
}