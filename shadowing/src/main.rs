/*
    Variable shadowing adalah istilah untuk variable yang dideklarasikan dalam sebuah block,
    yang variabel tersebut memiliki nama sama persis dengan variable pendahulunya
    (baik dalam scope yang sama ataupun variable lain yang berada di luar current scope)
*/

fn main() {
    let some_data = "Hello";
    println!("{}", some_data);
    // output => Hello

    let some_data = 12;
    println!("{}", some_data);
    // output => 12

    let some_data = "Rust!";
    println!("{}", some_data);
    // output => Rust!

    let some_data = false;
    println!("{}", some_data);

    let some_data = 3.14;
    println!("{}", some_data);
    // output => 3.14

    /*
        Variable shadowing BERBEDA dengan variable mutable
        Pada variable mutable, saat deklarasinya, di belakang layar terjadi proses alokasi alamat memory untuk menampung data,
        kemudian saat ada perubahan nilai, maka data yang baru disimpan ke alamat memory yang sama menggantikan data sebelumnya
    */

    /*
        Pada variable shadowing, yang terjadi di balik layar adalah:
        ketika ada deklarasi variabel baru menggunakan keyword let dan namanya sama, maka dianggap sebagai variabel baru,
        dan Rust akan mengalokasikan alamat memory baru untuk menampung data variable baru tersebut.
    */

    // Shadowing pada block berbeda
    let name = "Orgrim Doomhammer";
    let mut race = "Orc";
    let mut number = 27;

    println!("{}\t {}\t {}", name, race, number);
    // output => Orgrim Doomhammer        Orc     27

    {
        let name = "Sylvanas Windrunner";
        race = "Undead";
        let number = 24;

        println!("{}\t {}\t {}", name, race, number);
        // output => Sylvanas Windrunner      Undead  24
    }

    println!("{}\t {}\t {}", name, race, number);
    // output => Orgrim Doomhammer        Undead  27
}
