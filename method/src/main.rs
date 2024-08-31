// Method adalah associated item yang hanya bisa diakses lewat instance/object,
// berbeda dengan associated function yang pengaksesan fungsinya via tipe data struct.

// associated function (::)
// method (.)

mod models;

fn main() {
    let mut car = models::Car::new(
        String::from("Mercedes-Benz"),
        String::from("Vision Gran Turismo"),
    );
    println!("car: {:?}", car);

    // mengakses method
    let info = car.info();
    println!("info: {:?}", info);

    car.set_manufacture_year(2013);
    println!("detailed info: {:?}", car.info());

    car.congratulate(String::from("Sylvanas Windrunner"));
}
