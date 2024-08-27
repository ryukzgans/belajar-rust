mod my_io;
mod my_number;

// bisa juga seperti ini
// use my_number::conversion_utility::string_to_number;

fn main() {
    println!("enter any number: ");
    let message = my_io::read_enty(); // contoh modules lgsung ke parent
    println!("your message: {}", message);

    let number = my_number::conversion_utility::string_to_number(message); // contoh submodules
    let result = my_number::is_odd_number(number); // contoh modules lgsung ke parent dengan tambahan folder
    println!("is odd number: {}", result);

    println!("4x4: {}", my_number::perkalian::multiply(4, 4));

    /*
        Summary Modules
        - Penggunaan normal module dalam binary crate
        - Pembuatan module dengan nama my_io, dengan isi/item ditulis pada file my_io.rs
        - Pembuatan module dengan nama my_number, dengan isi/item ditulis pada file my_number/mod.rs
        - Penggunaan keyword pub untuk meng-export atau menjadikan item menjadi public, agar bisa diakses dari luar module
        - Pengaksesan item milik module, yaitu: my_io::read_entry, my_number::string_to_number, dan my_number::is_odd_number
    */

    /*
        Summary Submodules
        - Penggunaan normal module dalam binary crate
        - Pembuatan module dengan nama my_io, dengan isi/item ditulis pada file my_io.rs
        - Pembuatan module dengan nama my_number, dengan isi/item ditulis pada file my_number/mod.rs
        - Pembuatan submodule dengan nama my_number/conversion_utility, dengan isi/item ditulis pada file my_number/conversion.rs yang di-import menggunakan path attribute.
        - Penggunaan keyword pub pada fungsi agar bisa diakses dari luar module
        - Penggunaan keyword pub pada submodule agar bisa diakses dari luar parent module
        - Pengaksesan item milik module, yaitu: my_io::read_entry, my_number::conversion_utility::string_to_number, dan my_number::is_odd_number
    */

    /*
        Summary Path
        - Penggunaan normal module dalam binary crate
        - Pembuatan module dengan nama my_io, dengan isi/item ditulis pada file my_io.rs
        - Pembuatan module dengan nama my_number, dengan isi/item ditulis pada file my_number/mod.rs
        - Pembuatan submodule dengan nama my_number/conversion_utility, dengan isi/item ditulis pada file my_number/conversion_utility/mod.rs
        - Penggunaan keyword pub pada fungsi agar bisa diakses dari luar module
        - Penggunaan keyword pub pada submodule agar bisa diakses dari luar parent module
        - Pengaksesan item milik module, yaitu: my_io::read_entry, my_number::conversion_utility::string_to_number, dan my_number::is_odd_number
    */
}
