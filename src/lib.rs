pub mod sample_trait {
    pub trait Shape {
        fn calc_area(&self) -> f64;
        fn calc_permimeter(&self) -> f64;
        fn do_something();
    }
    pub struct Rectangle {
        pub width: f64,
        pub height: f64,
    }

    impl Shape for Rectangle {
        fn calc_area(&self) -> f64 {
            self.width * self.height
        }
        fn calc_permimeter(&self) -> f64 {
            2.0 * (self.width + self.height)
        }
        fn do_something() {
            println!("do something");
        }
    }

    pub struct Circle {
        pub radius: f64,
    }

    impl Shape for Circle {
        fn calc_area(&self) -> f64 {
            std::f64::consts::PI * self.radius * self.radius
        }
        fn calc_permimeter(&self) -> f64 {
            2.0 * std::f64::consts::PI * self.radius
        }
        fn do_something() {
            println!("do something");
        }
    }
}
