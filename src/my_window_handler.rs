use speedy2d::color::Color;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::Graphics2D;
use crate::body::Body;
use crate::universal_gravitation::gravitation::calculate_gravitation;

pub struct MyWindowHandler {
    pub bodies: Vec<Body>
}

impl MyWindowHandler {
    pub fn apply_gravitation_to_bodies(bodies: &mut Vec<Body>) {
        let bodies_count = bodies.len();
        if bodies_count > 1 {
            for i in 0..bodies_count-1 {
                for j in i+1..bodies_count {
                    println!("i: {}\tj:{}",i,j);
                    let mut ab1 = 0.0;
                    let mut ab2 = 0.0;

                    match (bodies.get(i), bodies.get(j)) {
                        (Some(b1), Some(b2)) => {
                            ab1 = calculate_gravitation(&b1, &b2, 10.0);
                            ab2 = calculate_gravitation(&b2, &b1, 10.0);
                        }
                        _ => {}
                    }
                    bodies[i].a = ab1;
                    bodies[j].a = ab2;
                    println!("ob1 {}\tob2 {}", bodies[i].a, bodies[j].a);
                }
            }
        }

    }
}

impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper<()>, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::from_rgb(0.1, 0.1, 0.1));

        //graphics.draw_circle((100.0, 100.0), 75.0, Color::BLUE);
        Self::apply_gravitation_to_bodies(&mut self.bodies);
        for body in &mut self.bodies {
            body.draw(graphics);
        }

        // Request that we draw another frame once this one has finished
        helper.request_redraw();
    }
}