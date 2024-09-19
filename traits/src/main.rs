// Traits itu seperti menerapkan sifat dari method lain ke tipe data kita contohnya struct

// #[derive(Debug)] // agar bisa di print kita wajib implementasi hal berikut
struct Circle {
    radius: i32,
}

// contoh implementasi trait pada struct Circle, tujuannya agar struct tersebut dapat di print menggunakan {:?}
impl std::fmt::Debug for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Circle radius: {}", self.radius)
    }
}

// contoh implementasi untuk dapat di print menggunakan {}
impl std::fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Circle radius: {}", self.radius)
    }
}

fn main() {
    let circle_one = Circle { radius: 6 };
    println!("{:?}", circle_one);
    println!("{}", circle_one);
}
