pub mod solar_system {
    use speedy2d::color::Color;
    use crate::body::Body;

    /**
     * Object       Diameter        Mass                    Distance
     * SUN          1392700         19891 * 10^30           0
     * MERCURY      4879.4          33011 * 10^23           57910000
     * VENUS        12104.0         48685 * 10^24           108200000
     * EARTH        12742.0         59736 * 10^24           149600000
     * MARS         6779.0          64174 * 10^23           227940000
     */

    /*const PROPORTION: f32 = 1.0;

    const SUN_DIAMETER: f32 = 1392700.0;
    const MERCURY_DIAMETER: f32 = 4879.4;
    const VENUS_DIAMETER: f32 = 12104.0;
    const EARTH_DIAMETER: f32 = 12742.0;
    const MARS_DIAMETER: f32 = 6779.0;

    const SUN_MASS: f32 = 19891.0 * 10.0_f32.powf(30.0);
    const MERCURY_MASS: f32 = 33011.0 * 10.0_f32.powf(23.0);
    const VENUS_MASS: f32 = 48685.0 * 10.0_f32.powf(24.0);
    const EARTH_MASS: f32 = 59736.0 * 10.0_f32.powf(24.0);
    const MARS_MASS: f32 = 64174.0 * 10.0_f32.powf(23.0);

    const MERCURY_SUN_DISTANCE: f32 = 57910000.0;
    const VENUS_SUN_DISTANCE: f32 = 108200000.0;
    const EARTH_SUN_DISTANCE: f32 = 149600000.0;
    const MARS_SUN_DISTANCE: f32 = 227940000.0;*/

    /*
        let sun = Body::new(900.0, 500.0, 1989.0000, 0.0, 139.20, 0.0, 0.0, Color::YELLOW);
        let mercury = Body::new(743.0, 343.0, 0.3, 0.0, 4.8, -30.0, 0.0, Color::GRAY);
        let venus = Body::new(692.0, 292.0, 4.8, 0.0, 12.1, -30.0, 0.0, Color::from_rgb(1.0, 0.6, 0.0));
        let earth = Body::new(1149.0, 749.0, 5.9, 0.0, 12.7, 30.0, 0.0, Color::BLUE);
        let mars = Body::new(1297.0, 827.0, 0.6, 0.0, 6.7, 30.0, 1.3, Color::RED);
    */
    pub fn create_first_five() -> Vec<Body> {
        vec![
            Body::new(String::from("sun"),900.0, 500.0, 1989.0, 0.0, 139.20, 0.0, 0.0, Color::YELLOW),
            Body::new(String::from("mercury"),693.0, 500.0, 0.003, 0.0, 4.8, 0.0, 0.8, Color::GRAY),
            Body::new(String::from("venus"),542.0, 500.0, 0.048, 0.0, 12.1, 0.0, 0.6, Color::from_rgb(1.0, 0.6, 0.0)),
            Body::new(String::from("earth"),401.0, 500.0, 0.059, 0.0, 12.7, 0.0, 0.5, Color::BLUE),
            Body::new(String::from("mars"),100.0, 500.0, 0.006, 0.0, 6.7, 0.0, 0.4, Color::RED)
        ]
    }

    /*
    Body::new(String::from("sun"),900.0, 500.0, 1989.0, 0.0, 139.20, 0.0, 0.0, Color::YELLOW),
            //Body::new(String::from("mercury"),693.0, 500.0, 0.003, 0.0, 4.8, 0.0, 0.5, Color::GRAY),
            //Body::new(String::from("venus"),542.0, 500.0, 0.048, 0.0, 12.1, 0.0, 0.4, Color::from_rgb(1.0, 0.6, 0.0)),
            Body::new(String::from("earth"),401.0, 500.0, 0.059, 0.0, 12.7, 0.0, 0.0, Color::BLUE),
            //Body::new(String::from("mars"),100.0, 500.0, 0.006, 0.0, 6.7, 0.0, 0.2, Color::RED)
            */
}