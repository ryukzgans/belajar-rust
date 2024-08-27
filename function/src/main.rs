fn main() {
    println!("Hello Dunia");

    greet();
    greet_custom_message("Ilham", "Halo gays");
    register_user("ilhamkurniawan@gmail.com", "ilham kurniawan", 24);

    let res1 = calculate_box_volume1(5, 8, 12);
    println!("result1: {}", res1);

    let res2 = calculate_box_volume2(5, 8, 12);
    println!("resutl2: {}", res2);

    // Macro format untuk membuat formatted string
    let res3 = calculate_box_volume3(5, 8, 12);
    let message3 = format!("the box volume is {}", res3);
    greet_custom_message("Ilham", message3.as_str());

    println!("{}", get_score_message(100.0));
    println!("{}", get_score_message(95.2));
    println!("{}", get_score_message(33.12));
}

// Function
fn greet() {
    println!("Hello World")
}

//// Function with parameter
// fn func_name(param_a: type, param_b: type) {}

fn greet_custom_message(name: &str, message: &str) {
    println!("{}: {}", name, message);
}

fn register_user(email: &str, name: &str, age: i32) {
    println!("Registration Successfully!");
    println!("Welcome {} with {}, youre {} old", name, email, age);
}

//// Nilai balik fungsi (return value)
// fn func_name(param_a: type, param_b: type) -> type {}

fn calculate_box_volume1(width: i32, height: i32, length: i32) -> i32 {
    let volume = width * height * length;
    return volume;
}

// return value dapat dilakukan tanpa keyword "return"
fn calculate_box_volume2(width: i32, height: i32, length: i32) -> i32 {
    let volume = width * height * length;
    volume // tanpa return dan ;
}

// contoh lain tanpa keyword return
fn calculate_box_volume3(width: i32, height: i32, length: i32) -> i32 {
    width * height * length
}

// contoh lain, kapan bisa digunakan tanpa keyword return
fn get_score_message(score: f32) -> &'static str {
    if score == 100.0 {
        return "you got a perfect score";
    }

    if score > 76.0 {
        return "congrats, you passed the exam!";
    }

    "your score is below the passing grade" // tanpa keyword return hanya bisa dipergunakan di akhir blok kode
}
