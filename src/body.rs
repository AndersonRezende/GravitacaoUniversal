use std::fmt::format;
use std::path::Path;
use speedy2d::color::Color;
use speedy2d::Graphics2D;
use speedy2d::image::{ImageHandle, ImageSmoothingMode};

#[derive(Debug)]
pub struct Body {
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub m: f32,
    pub a: f32,
    pub radius: f32,
    pub color: Color,

    pub x_speed: f32,
    pub y_speed: f32,
    image: Option<ImageHandle>,
}

impl Body {
    pub fn new(name: String, x: f32, y: f32, m: f32, a: f32, radius: f32, x_speed: f32, y_speed: f32, color: Color) -> Body {
        Body { name, x, y, m, a, radius, color, x_speed, y_speed, image: Option::None }
    }

    fn draw_image(&mut self, graphics2d: &mut Graphics2D) {
        let path = format!("{}{}{}", String::from("assets/"), self.name, String::from(".png"));
        if Path::new(&path).exists() && false {
            if self.image.is_none() {
                let image =
                    graphics2d.create_image_from_file_path(
                        None,
                        ImageSmoothingMode::NearestNeighbor,
                        path
                    ).unwrap();
                self.image = Some(image);
            }
            graphics2d.draw_image((self.x, self.y), self.image.as_ref().unwrap());
        }
    }

    pub fn draw(&mut self, graphics2d: &mut Graphics2D) {
        graphics2d.draw_circle((self.x, self.y), self.radius, self.color);
        //self.draw_image(graphics2d);
    }

    pub fn update(&mut self) {
        self.x += self.x_speed;
        self.y += self.y_speed;
    }
}