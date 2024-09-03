/*
    # Ownership
    Ownership merupakan kumpulan aturan yang ada di Rust yang dijadikan acuan oleh compiler dalam pengelolahan memory.

    # Aturan Ownership
    Ada 3 aturan penting yang wajib diketahui:
    - Semua nilai/data/value di Rust memiliki owner. Misal kita berbicara tentang deklarasi variabel dengan predefined value,
      maka value variabel adalah yang dimaksud dengan nilai/data/value, dan variabel itu sendiri adalah owner dari
      nilai/data/value tersebut.
    - Pada waktu yang sama, hanya boleh ada 1 owner. Satu data, ownernya hanya satu.
    - Ketika eksekusi sebuah block scope selesai, maka owner dari data-data yang ada dalam scope tersebut
      akan di-drop atau di-dealokasi (dengan pengecualian yaitu owner berpindah ke luar scope).

      more information
      https://dasarpemrogramanrust.novalagung.com/basic/ownership
*/

fn main() {
    let msg1 = String::from("hello");
    let msg2 = msg1;
    let msg3 = msg2;
    // let msg4 = msg2; -> error karena data msg2 sudah invalid

    // println!("msg1: {}", msg1); -> error karena data msg1 sudah dipinjam/borrow by msg2
    // println!("msg2: {}", msg2); -> error karena data msg2 sudah dipinjam/borrow by msg3
    println!("msg3: {}", msg3); // -> ok

    ///////
    // let msg_new = String::from("Hello World");
    // say_hello(msg_new);
    // println!("{:?}", msg_new);

    // pada contoh diatas akan menghasilkan error pada println terakhir dikarenakan data msg_new sudah di borrow ke say_hello
    // salah satu solusinya bisa seperti dibawah ini
    let msg_new = String::from("Hello World");
    say_hello(msg_new.clone()); // dengan clone, tetapi cara ini tetap saja bukan cara terbaik untuk memory management
    println!("{:?}", msg_new);
}

fn say_hello(param: String) {
    println!("{:?}", param);
}
