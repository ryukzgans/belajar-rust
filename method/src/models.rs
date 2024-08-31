#[derive(Debug)]
pub struct Car {
    brand: String,
    model: String,
    manufacture_year: i32,
}

impl Car {
    pub fn new(brand: String, model: String) -> Self {
        Self {
            brand,
            model,
            manufacture_year: 0,
        }
    }

    // deklarasi method harus diisi dengan &self
    pub fn info(&self) -> String {
        if self.manufacture_year == 0 {
            format!("{} model {}", self.brand, self.model)
        } else {
            format!(
                "{} model {}, manufacture at {}",
                self.brand, self.model, self.manufacture_year
            )
        }
    }

    pub fn congratulate(&self, name: String) {
        println!("Hello {}", name);
        println!("Congrats with your new car {}", self.info());
        println!("vroooommmm vrooooooommmm!");
    }

    // mutable struct property
    pub fn set_manufacture_year(&mut self, year: i32) {
        self.manufacture_year = year
    }
}
