// static (static), wajib diluar function
static NUMBER: i32 = 18;

fn main() {
    // Konstanta (const), uppercase dan wajib mendefinisikan tipe data, snakecase
    const LABEL: &str = "Nilai PI adalah:";
    const PI: f32 = 22.0 / 7.0;

    println!("{} {:.2}", LABEL, PI);

    println!("{}", NUMBER);
}
