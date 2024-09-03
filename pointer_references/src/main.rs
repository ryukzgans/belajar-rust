/*
    # Pointer
    Pointer artinya adalah alamat memori. Variabel pointer artinya adalah variabel yang berisi
    alamat memory (hanya alamat memory-nya saja, bukan value yang sebenarnya)

    # Reference (operator &)
    Reference artinya adalah pointer dari sebuah variabel atau data. Operasi pengambilan pointer dari variabel disebut
    dengan referencing, caranya dilakukan dengan menggunakan karakter &

    # Dereference (operator *)
    Dereference artinya mengambil nilai sebenarnya atau underlying value dari sebuah pointer (istilahnya adalah dereference).

    # Mutable References (operator &mut)
    By default, reference sifatnya read-only atau immutable, artinya tidak bisa diubah underlying-value-nya.

    ## Aturan Reference
    Ada dua aturan penting yang harus dipatuhi dalam penerapan reference baik mutable atau immutable reference.
    - Dalam waktu yang sama, hanya boleh ada satu mutable reference atau banyak immutable reference (keduanya tidak bisa bersamaan, harus salah satu).
    - Reference harus selalu valid.

*/

use rand::Rng;

fn main() {
    let number: i32 = 24;
    println!("value: {:?}", number);

    // variabel pointer_number nilainya adalah reference variabel number.
    // contoh statement:
    let pointer_number: &i32 = &number;
    println!("pointer value: {:?}", pointer_number);
    println!("memory address: {:p}", pointer_number); // return memory address

    // dereference (operator *)
    let underlying_value = *pointer_number;
    println!("underlying_value: {:}", underlying_value);

    println!();
    // mutable reference (operator &mut)
    let mut number_two = 24;
    println!("number two: {:?}", number_two);

    let pointer_number_two = &mut number_two; // &mut
    println!("pointer value two: {:?}", pointer_number);

    *pointer_number_two = 12;
    println!("pointer number two: {:?}", pointer_number_two);
    println!("number two: {}", number_two);

    //////
    println!();

    // Example alokasi memory dengan pointer
    let mut number_three = 24;
    println!("number_three: {}", number_three);

    for _ in 0..=5 {
        change_value(&mut number_three);
        println!("number_three: {}", number_three);
    }

    /*
        Di Rust, reference (atau pengaksesan alamat memory suatu data) memiliki hubungan yang sangat erat dengan konsep borrowing.
        Ketika kita mengambil reference suatu data, yang terjadi sebenarnya adalah kita meminjam data tersebut dari owner/pemilik sebenarnya.
    */
}

fn change_value(n: &mut i32) {
    *n = generate_random_number()
}

fn generate_random_number() -> i32 {
    rand::thread_rng().gen_range(0..100)
}
