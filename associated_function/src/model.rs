// implementation tuple struct
#[derive(Debug)]
pub struct Color(i32, i32, i32);

impl Color {
    pub fn new(r: i32, g: i32, b: i32) -> Self {
        Self(r, g, b)
    }

    pub fn red() -> Self {
        Self(255, 0, 0)
    }

    pub fn green() -> Self {
        Self(0, 255, 0)
    }

    pub fn blue() -> Self {
        Self(0, 0, 255)
    }
}
