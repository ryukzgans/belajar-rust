pub struct Circle {
    pub radius: i32,
}

impl crate::calculation_spec::Area for Circle {
    fn calculate_area(&self) -> f64 {
        // PI * (r ^ 2)
        // ada operasi casting ke tipe f64 karena self.radius bertipe i32
        3.14 * (self.radius.pow(2) as f64)
    }
}
impl crate::calculation_spec::Circumference for Circle {
    fn calculate_circumference(&self) -> f64 {
        2.0 * 3.14 * (self.radius) as f64
    }
}

pub struct Square {
    pub length: i32,
}

impl crate::calculation_spec::Area for Square {
    fn calculate_area(&self) -> f64 {
        // (s ^ 2)
        // ada operasi casting ke tipe f64 karena self.length bertipe i32
        self.length.pow(2) as f64
    }
}
impl crate::calculation_spec::Circumference for Square {
    fn calculate_circumference(&self) -> f64 {
        4.0 * (self.length) as f64
    }
}
