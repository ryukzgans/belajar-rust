use calculation_spec::{Area, Circumference};

mod calculation_spec;
mod two_dimensional;

fn main() {
    let circle_one = two_dimensional::Circle { radius: 10 };
    println!("Circle area: {}", circle_one.calculate_area());

    let square_one = two_dimensional::Square { length: 5 };
    println!("Square area: {}", square_one.calculate_area());

    println!();
    // example using trait sebagai tipe param
    let circle_two = two_dimensional::Circle { radius: 15 };
    calculate_and_print_result("circle".to_string(), &circle_two);

    let square_two = two_dimensional::Square { length: 10 };
    calculate_and_print_result("square".to_string(), &square_two);

    println!();
    // Example trait sebagai return type
    let circle_three = new_circle(5);
    calculate_and_print_result5("circle".to_string(), &circle_three);

    let square_three = new_square(10);
    calculate_and_print_result6("square".to_string(), &square_three);
}

// Trait sebagai tipe parameter
// fn calculate_and_print_result(name: String, item: &impl Area) {
//     println!("{} area: {}", name, item.calculate_area())
// }

// Parameter bertipe lebih dari 1 trait
fn calculate_and_print_result(name: String, item: &(impl Area + Circumference)) {
    println!("{} area: {}", name, item.calculate_area());
    println!("{} circumference: {}", name, item.calculate_circumference());
}

// Trait bound syntax
fn calculate_and_print_result2<T: Area>(name: String, item: &T) {
    println!("{} area: {}", name, item.calculate_area());
}

// Trait bound syntax, lebih dari 1 trait
fn calculate_and_print_result3<T: Area + Circumference>(name: String, item: &T) {
    println!("{} area: {}", name, item.calculate_area());
    println!("{} circumference: {}", name, item.calculate_circumference());
}

// Trait where clause
fn calculate_and_print_result4<T>(name: String, item: &T)
where
    T: Area + Circumference,
{
    println!("{} area: {}", name, item.calculate_area());
}

// Trait sebagai return type
// new_circle dengan return type adalah impl Area
fn new_circle(radius: i32) -> impl Area {
    two_dimensional::Circle { radius }
}

// new_square dengan return type adalah impl Area + Circumference
fn new_square(length: i32) -> impl Area + Circumference {
    two_dimensional::Square { length }
}

fn calculate_and_print_result5<T>(name: String, item: &T)
where
    T: Area,
{
    println!("{} area: {}", name, item.calculate_area());
}

fn calculate_and_print_result6<T>(name: String, item: &T)
where
    T: Area + Circumference,
{
    println!("{} area: {}", name, item.calculate_area());
    println!("{} circumference: {}", name, item.calculate_circumference());
}
