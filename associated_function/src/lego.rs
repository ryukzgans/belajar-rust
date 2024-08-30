// example module implementation

#[derive(Debug)]
pub struct LegoSet {
    code: i32,
    name: String,
    category: String,
    age_minimum: i32,
}

// Untuk blok kode impl tidak perlu ditambahi keyword pub
impl LegoSet {
    pub fn new(code: i32, name: String, category: String, age_minimum: i32) -> Self {
        Self {
            code,
            name,
            category,
            age_minimum,
        }
    }

    pub fn what_is_lego() {
        println!("Lego is a line of plastic construction toys")
    }
}
