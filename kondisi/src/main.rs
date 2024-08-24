fn main() {
    // If, else if, else statement
    let number_a: i8 = 20;

    if number_a == 10 {
        println!("number a = 10")
    } else if number_a < 10 {
        println!("number a lebih kecil dari 10")
    } else {
        println!("number a lebih besar dari 10")
    }

    // Nested if
    let number_c = 10;
    if number_c > 6 {
        println!("selamat, anda lulus");

        if number_c == 10 {
            println!("dengan nilai sempurna");
        } else if number_c > 7 {
            println!("dengan nilai baik");
        } else {
            println!("dengan nilai cukup");
        }
    } else {
        println!("anda tidak lulus");

        if number_c < 4 {
            println!("belajar lagi ya");
        } else {
            println!("jangan malas belajar!");
        }
    }

    // Returning from if
    let number_d = 3;
    let result_d: bool;

    if number_d == 2 {
        result_d = true;
    } else {
        result_d = false;
    }
    println!("result_d adalah {result_d}");

    // Contoh let dan if
    let result_e = if number_d == 2 { true } else { false };
    println!("result_e adalah {result_e}");

    // Contoh let dan if dengan data explicit
    let result_f: &str = if number_d == 2 {
        "angka adalah 2"
    } else if number_d < 2 {
        "angka lebih kecil dari 2"
    } else {
        "angka lebih besar dari 2"
    };
    println!("result_f adalah {result_f}");

    // Contoh lain
    let number_g = 100.0;
    let string_g = "nilai minimum kelulusan";
    let result_g: f64 = if string_g == "nilai minimum kelulusan" {
        number_g
    } else {
        number_g * 3.0 / 4.0
    };

    println!("angka adalah {result_g}");
}
