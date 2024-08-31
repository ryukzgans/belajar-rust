mod constants;

// Enum atau enumerated type adalah sebuah tipe data yang digunakan untuk menampung nilai konstan

// definisi konstanta
const SUPERHEROSUPERMAN: &str = "superman";
const SUPERHEROOMNIMAN: &str = "omniman";
const SUPERHEROHOMELANDER: &str = "homelander";
const SUPERHEROHYPERION: &str = "hyperion";

// definisi enum
#[derive(Debug)]
enum Superhero {
    Superman,
    OmniMan,
    Homelander,
    Hyperion,
}

fn main() {
    // Definisi variabel dengan isi konstanta
    let value1: &str = SUPERHEROSUPERMAN;
    let value2 = SUPERHEROOMNIMAN;

    // Definisi variabel bertipe data enum SuperHero
    let value3: Superhero = Superhero::Superman;
    let value4 = Superhero::OmniMan;

    println!("value1: {value1}");
    println!("value2: {value2}");
    println!("value3: {:?}", value3);
    println!("value4: {:?}", value4);

    enum Food {
        PenyetanTerangBulan,
        PizzaNanas,
        EsKrimIkanMujaer,
        MiGorengKuah,
        MakananLainnya(String), // enum tuple struct, value baru
        MieSetan {
            level_pedes: i32,
            pakek_piring: bool,
        }, // enum struct like
    }

    // Seleksi pada enum hanya dapat dilakukan menggunakan match

    // example 1
    let makanan_favorit = Food::PenyetanTerangBulan;
    match makanan_favorit {
        Food::PenyetanTerangBulan => {
            println!("your food taste is quite ... unique");
        }
        Food::PizzaNanas => {
            println!("it's morally wrong to have pineapple on top of pizza");
        }
        Food::EsKrimIkanMujaer => {
            println!("i don't know what to say");
        }
        Food::MiGorengKuah => {
            println!("sometimes people do eat this, but it's ok");
        }
        _ => {
            println!("never heard about that food")
        }
    }

    // example 2
    let nasi_goreng = Food::MakananLainnya(String::from("nasi goreng"));
    match nasi_goreng {
        Food::PenyetanTerangBulan => {
            println!("your food taste is quite ... unique");
        }
        Food::PizzaNanas => {
            println!("it's morally wrong to have pineapple on top of pizza");
        }
        Food::EsKrimIkanMujaer => {
            println!("i don't know what to say");
        }
        Food::MiGorengKuah => {
            println!("sometimes people do eat this, but it's ok");
        }
        // seleksi kondisi baru m
        Food::MakananLainnya(m) => {
            println!("do you like {m}? nice taste!");
        }
        _ => {
            println!("never heard about that food")
        }
    }

    // example 3
    let miesetan = Food::MieSetan {
        level_pedes: 5,
        pakek_piring: false,
    };
    match miesetan {
        Food::PenyetanTerangBulan => {
            println!("your food taste is quite ... unique");
        }
        Food::PizzaNanas => {
            println!("it's morally wrong to have pineapple on top of pizza");
        }
        Food::EsKrimIkanMujaer => {
            println!("i don't know what to say");
        }
        Food::MiGorengKuah => {
            println!("sometimes people do eat this, but it's ok");
        }
        // seleksi kondisi baru m
        Food::MakananLainnya(m) => {
            println!("do you like {m}? nice taste!");
        }
        Food::MieSetan {
            level_pedes,
            pakek_piring,
        } => {
            if level_pedes > 3 {
                println!("mie setan lvl {} is too much!", level_pedes);
            } else {
                println!("mie setan lvl {} is perfect!", level_pedes);
            }

            if !pakek_piring {
                println!("how are you going to eat the food without a plate, huh?")
            }
        }
    }

    // Enum module & visibility
    let company = constants::Company::Apple;

    match company {
        constants::Company::Apple => {
            println!("apple")
        }
        _ => {
            println!("other than apple")
        }
    }
}
