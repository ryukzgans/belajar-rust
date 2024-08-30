// using module
mod lego;
mod model;

// inline implementation
#[derive(Debug)]
struct LegoSet {
    code: i32,
    name: String,
    category: String,
    age_minimum: i32,
}
// Keyword impl digunakan untuk membuat associated item
impl LegoSet {
    fn new(code: i32, name: String, category: String, age_minimum: i32) -> LegoSet {
        LegoSet {
            code,
            name,
            category,
            age_minimum,
        }
    }

    // with Self (tipe data Self hanya bisa digunakan dalam blok kode impl)
    fn new_one(code: i32, name: String, category: String, age_minimum: i32) -> LegoSet {
        Self {
            code,
            name,
            category,
            age_minimum,
        }
    } // example 1

    fn new_two(code: i32, name: String, category: String, age_minimum: i32) -> Self {
        LegoSet {
            code,
            name,
            category,
            age_minimum,
        }
    } // example 2

    fn new_three(code: i32, name: String, category: String, age_minimum: i32) -> Self {
        Self {
            code,
            name,
            category,
            age_minimum,
        }
    } // example 3

    fn what_is_lego() {
        println!("Lego is a line of plastic construction toys")
    }
}

fn main() {
    // definisi struct biasa
    let rough_terrain_crane = LegoSet {
        code: 42082,
        name: String::from("Rough Terrain Crane"),
        category: String::from("Technic"),
        age_minimum: 11,
    };

    println!("{:#?}", rough_terrain_crane);

    // NamaStruct::nama_fungsi(arg1, arg2, arg3, arg4);
    let xtreme_offroader = LegoSet::new(
        42099,
        String::from("4X4 X-treme Off-Roader"),
        String::from("Technic"),
        11,
    );

    LegoSet::what_is_lego();
    println!("{:#?}", xtreme_offroader);

    // implementasi using module
    let object_new = lego::LegoSet::new(
        42099,
        String::from("4X4 X-treme Off-Roader"),
        String::from("Technic"),
        12,
    );
    println!("{:#?}", object_new);

    lego::LegoSet::what_is_lego();

    println!();
    // implementasi tuple struct
    let red_color = model::Color::red();
    let green_color = model::Color::green();
    let blue_color = model::Color::blue();

    println!(
        "red: {:?},\ngreen: {:?},\nblue: {:?}",
        red_color, green_color, blue_color
    );

    let custom_color = model::Color::new(12, 25, 47);
    println!("custom color: {:?}", custom_color);
}
