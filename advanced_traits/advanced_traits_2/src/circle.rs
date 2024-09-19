// Disiapkan struct circle::Circle yang mengadopsi trait shape::Shape

pub struct Circle {
    pub radius: f64,
}

impl crate::shape::Shape for Circle {
    type Area = f64;

    fn area(&self) -> Self::Area {
        std::f64::consts::PI * self.radius * self.radius
    }
}

// Contoh di atas adalah cara pengaplikasian associated types.
