#[path = "multiply.rs"] // path
pub mod perkalian;

pub mod conversion_utility;

pub fn is_odd_number(number: i32) -> bool {
    number % 2 == 1
}
