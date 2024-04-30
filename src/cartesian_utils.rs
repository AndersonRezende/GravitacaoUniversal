pub mod cartesian {
    pub fn angle_between_two_points(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
        let x: f32 = x2 - x1;
        let y: f32 = y2 - y1;

        let aux: f32 = (x.powi(2) + y.powi(2)).sqrt();
        let dot_product_x: f32 = x / aux;
        let dot_product_y: f32 = y / aux;
        let angle_rad: f32 = dot_product_y.atan2(dot_product_x);
        angle_rad.to_degrees()
    }



    pub fn distance(x1: f32, x2: f32, y1: f32, y2: f32) -> f32 {
        ((x2 - x1).powf(2.0) + (y2 - y1).powf(2.0)).sqrt()
    }
}