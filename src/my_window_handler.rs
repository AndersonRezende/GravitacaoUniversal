use std::f32::consts::PI;
use speedy2d::color::Color;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::Graphics2D;
use crate::body::Body;
use crate::cartesian_utils::cartesian::angle_between_two_points;
use crate::universal_gravitation::gravitation::calculate_gravitation;

pub struct MyWindowHandler {
    pub bodies: Vec<Body>
}

impl MyWindowHandler {
    fn apply_gravitation_to_bodies(bodies: &mut Vec<Body>) {
        let bodies_count = bodies.len();
        if bodies_count > 1 {
            for i in 0..bodies_count-1 {
                for j in i+1..bodies_count {
                    let mut ab1 = 0.0;
                    let mut ab2 = 0.0;

                    match (bodies.get(i), bodies.get(j)) {
                        (Some(b1), Some(b2)) => {
                            ab1 = calculate_gravitation(&b1, &b2, 10.0);
                            ab2 = calculate_gravitation(&b2, &b1, 10.0);
                        }
                        _ => {}
                    }

                    let direction1: f32 = angle_between_two_points(bodies[i].x, bodies[i].y, bodies[j].x, bodies[j].y);
                    let direction2: f32 = angle_between_two_points(bodies[j].x, bodies[j].y, bodies[i].x, bodies[i].y);
                    bodies[i].x_speed += (direction1 * PI / 180.0).cos() * ab1;
                    bodies[i].y_speed += (direction1 * PI / 180.0).sin() * ab1;

                    bodies[j].x_speed += (direction2 * PI / 180.0).cos() * ab2;
                    bodies[j].y_speed += (direction2 * PI / 180.0).sin() * ab2;
                }
            }
        }
    }
}

impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper<()>, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::from_rgb(0.1, 0.1, 0.1));

        Self::apply_gravitation_to_bodies(&mut self.bodies);
        for body in &mut self.bodies {
            body.update();
            body.draw(graphics);
        }

        // Request that we draw another frame once this one has finished
        helper.request_redraw();
    }
}