
pub mod gravitation {
    use crate::body::Body;

    // G = 6,67408.10-11 N.m²/kg²
    const G: f32 = 6_67408.10 * 0.0000001;

    fn calculate_gravitation_force(body1: &Body, body2: &Body, d: f32) -> f32 {
        // |F| = (G * M * m) / d²
        let f = G * (body1.m * body2.m / d.powf(2.0));
        f.abs()
    }

    fn calculate_gravitation_force_aceleration(f: f32, m: f32) -> f32 {
        // f = m * a    =>  a = f / m
        f / m
    }

    pub fn calculate_gravitation(body1: &Body, body2: &Body, d: f32) -> f32 {
        let f: f32 = calculate_gravitation_force(body1, body2, d);
        calculate_gravitation_force_aceleration(f, body1.m)
    }
}

