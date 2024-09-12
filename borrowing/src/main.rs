fn main() {
    // let msg_1 = String::from("Hello");
    // let msg_2 = num1;

    // println!("message2 {}", msg_2);
    // println!("message1 {}", msg_1); // Error

    /*
        Borrowing artinya adalah meminjam. Pada konteks Rust programming, borrowing berarti meminjam data milik owner,
        dipinjam agar bisa diakses tanpa perlu memindah owner-nya. Kemudian setelah peminjaman selesai, data dikembalikan.
    */

    let msg_3 = String::from("Hello World");
    let msg_4 = &msg_3; // <----- borrow operation

    println!("{:?}", msg_4); // output => Hello World
    println!("{:?}", msg_3); // output => Hello World

    // Borrowing dengan level akses mutable
    let mut msg_6 = String::from("Hello New");
    let msg_7 = &mut msg_6; // <----- mutable borrow operation

    *msg_7 = String::from("Hello Dunia"); // update

    println!("{:?}", msg_7); // output => Hello Dunia
    println!("{:?}", msg_6); // output => Hello Dunia

    /*
    Aturan Borrowing
    - Dalam waktu yang sama, hanya boleh ada satu mutable reference atau banyak immutable reference (keduanya tidak bisa bersamaan, harus salah satu).
    - Reference harus selalu valid.
    */

    println!();
    // Borrowing pada function
    let mut fact_one = String::from("Arthas is the true lich king");

    change_value(&mut fact_one);

    let fact_two = &mut fact_one;
    println!("{:?}", fact_two);
    // walaupun kode diatas terjadi dalam waktu yang sama, tetapi tidak terjadi error dikarenakan sudah berbeda scope

    println!();
    // Borrowing pada block
    let mut true_one = String::from("Arthas is the true lich king");
    println!("{:?}", true_one);

    // block 1.. this is block or what ? xD
    change_value(&mut true_one);
    println!("{:?}", true_one);

    // block 2
    {
        let true_two = &mut true_one;
        *true_two = String::from("There must always be a lich king");
        println!("{:?}", true_one);
    }

    // block 3
    if true_one.contains("lich king") {
        let true_three = &mut true_one;
        *true_three = String::from("Who is the real jailer?");
        println!("{:?}", true_one);
    }

    // block 4
    for _ in 0..1 {
        let true_four = &mut true_one;
        *true_four = String::from("Is it Zovaal or Primus?");
        println!("{:?}", true_one);
    }
}

fn change_value(txt: &mut String) {
    *txt = String::from("Bolvar is better lich king");
}
