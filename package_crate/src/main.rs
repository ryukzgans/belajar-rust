use rand::Rng;

fn main() {
    // Rust Crate
    /*
        Crate adalah satu unit kompilasi di Rust. Eksekusi command cargo run, cargo build, atau rustc men-trigger proses kompilasi, dan unit (yang di sini disebut dengan crate) akan di-compile.

        Crate bisa berisi banyak module. Sebuah module definisinya bisa berada di banyak file. Agar lebih jelas silakan perhatikan contoh berikut:
        - XYZ adalah sebuah crate, isinya ada dua module, yaitu module Mod_ABC dan module Mod_DEF.
        - Mod_ABC adalah module yang didefinisikan dalam crate XYZ, source code-nya berada di file bernama modul_a.rs.
        - Mod_DEF adalah module yang didefinisikan dalam crate XYZ, source code-nya berada di beberapa file module_b_one.rs dan module_b_two.rs.

        Dari contoh di atas, crate XYZ adalah 1 unit kompilasi, yang mana di dalam crate tersebut ada dua modules yaitu Mod_ABC dan Mod_DEF

        Rust mengkategorikan crate menjadi 2 jenis, binary crate dan library crate
    */

    // Binary crate
    /*
        Binary crate adalah program yang dikompilasi ke bentuk executable, untuk kemudian dijalankan, seperti program-program yang sudah kita buat menggunakan cargo create dan run menggunakan cargo run itu adalah contoh dari binary crate.
        Binary crate berada dalam sebuah package yang dibuat menggunakan command cargo create <nama_package> atau cargo create --bin <nama_package>, kedua command ini menjalankan perintah yang sama.
        Ciri khas dari binary crate adalah memiliki fungsi main, sebuah fungsi yang merupakan entrypoint program.
    */

    // Library crate
    /*
        Library crate berbeda dengan binary crate. Library crate tidak di-compile ke bentuk executable dan tidak memiliki fungsi main. Library crate digunakan untuk mendefinisikan set functionality yang reusable atau bisa digunakan di banyak project/package.
        Library crate di-import/digunakan dalam binary crate. Dalam proses kompilasinya, yang di-compile adalah binary crate. Library crate juga akan ikut dalam kompilasi tersebut.
        Sebagai contoh item Duration (yang sudah dipraktikkan pada chapter Perulangan ➜ while) dan stdin (pada chapter Module System ➜ Path & Item) adalah dua buah item milik crate Rust Standard Library atau std. Crate std ini akan sangat sering kita gunakan dalam package/project, isinya banyak sekali functionality untuk keperluan standar dalam Rust programming.
        Di komunitas Rust, ketika ada kata library atau crate maka yang dimaksud biasanya adalah library crate
        Kita bisa membuat library crate kemudian di-publish ke crates.io agar bisa digunakan banyak orang. Command cargo new --lib <nama_package> digunakan untuk membuat library crate.
    */

    // Rust Package
    /*
    Istilah package dalam Rust programming masih sama dengan package dalam pemrograman lain. Package adalah sebuah set yang berisi banyak functionality. Satu buah package bisa berisi satu atau banyak crates.
    Package di-manage oleh Cargo, yang merupakan package manager Rust. Command cargo new <nama_package> digunakan untuk membuat package.
    Command tersebut menghasilkan beberapa file yaitu src/main.rs yang isinya adalah kode program, dan juga file Cargo.toml yang isinya adalah informasi mengenai package tersebut.
    */

    // File Cargo.toml
    /*
        Blok package berisi berisi 3 buah field:
        - name isinya adalah nama package, sesuai dengan argument command cargo new <nama_package>.
        - version default-nya selalu 0.1.0, namun kita bisa ubah nilainya seiring berjalannya proses pengembangan aplikasi.
        - edition di sini me-refer ke edisi rust yang dipakai. Di ebook ini, rust versi 1.65.0 digunakan, dan edisi untuk versi tersebut adalah 2021.

        Blok dependencies default-nya berisi kosong. Jika kita menambahkan external dependencies atau crates, maka detailnya tercatat pada blok dependencies ini.
    */

    // Command cargo add
    /*
        Selain cara di atas, ada cara lain yang lebih ringkas untuk penambahan package, yaitu menggunakan command cargo add <nama_crate>. Command ini secara otomatis menambahkan package yang diinginkan ke file Cargo.toml kemudian mengunduhnya.
        Menambahkan package rand versi terbaru
        - cargo add rand
        Menambahkan package rand versi 0.8.5
        - cargo add rand@0.8.5
    */

    // Menggunakan dependecy
    for i in 0..5 {
        println!("random number {}: {}", i, generate_number());
    }
}

// Depedency rand
fn generate_number() -> i32 {
    rand::thread_rng().gen_range(0..100)
}
