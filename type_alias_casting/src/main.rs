use std::{
    fs::FileTimes,
    time::{SystemTime, UNIX_EPOCH},
};

// Tipe data Inch adalah tipe baru yang merupakan alias dari tipe u64
type Inch = u64;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

// Coordinate merupakan alias dari struct Point
type Coordinate = Point;

fn main() {
    // type alias
    let height: Inch = 10; // Inch == u64
    println!("height: {height:?}");

    let height_in_u64 = height as u64;
    println!("height_in_u64: {height_in_u64:?}");

    // casting tipe data & alias
    let p = Point { x: 0, y: 10 };
    println!("p: {:?}", p);

    let mut q: Coordinate = p as Coordinate;
    q.x = 12;
    println!("q: {:?}", q);

    let r: Point = q as Point;
    println!("r: {:?}", r);

    // Operasi assignment dan type casting pada custom type struct membuat owner-nya berpindah.
    // Perpindahan owner ini disebut dengan move semantics.

    // Casting antar tipe scalar
    let number = 32;
    println!("number: {number}");

    let number_in_u8 = number as u8;
    println!("number_in_u8: {number_in_u8}");

    let number_in_f64 = number as f64;
    println!("number_in_f64: {number_in_f64}");

    let new_number = 23.4 as f32;
    println!("new_number: {new_number}");

    let number_in_char = 65 as char; // A
    println!("number_in_char: {number_in_char}");

    // Konsekuensi casting tipe numerik
    let timestamp: u64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    println!("timestamp (u64): {timestamp}");
    println!("timestamp (as u16): {}", timestamp as u16); // hasilnya angka menjadi lebih kecil

    // di-cast lagi ke tipe u64, hasilnya tidak akan berubah dan tetap sesuai dengan angka sesudah dirubah
    println!("from u16 back to u64: {}", (timestamp as u16) as u64);
}
