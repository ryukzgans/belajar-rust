fn main() {
    // Module System -> Path & Item

    // Rust Paths
    /*
        Rust paths mirip seperti konsep filesystem path di sistem operasi. Seperti C:\Users\novalagung\Desktop di windows, atau /etc/nginx/conf.d/nginx.conf di Unix/Linux.
        Di Rust, path tidak menggunakan \ atau / sebagai separator, melainkan ::.

        std::time::Duration

        - Path std ➜ adalah path untuk crate bernama Rust Standard Library, isinya adalah sangat banyak item untuk keperluan umum di Rust programming. Lebih jelasnya akan dibahas pada chapter Rust standard library.
        - Path std::time ➜ adalah path untuk module bernama time, isinya banyak item yang berhubungan dengan operasi waktu/time.
        - Path std::time::Duration ➜ adalah path untuk struct bernama Duration, yang merupakan representasi dari unit waktu.
    */

    // Absolute & relative paths
    /*
        - Absolute path ➜ adalah path yang penulisannya lengkap dari root path, contohnya seperti std::time::Duration.
        - Relative path ➜ adalah path yang penulisannya relatif terhadap current path, contohnya seperti self::my_func, super::my_mod::my_consntan.
    */

    // Penerapan paths dalam penggaksesan item
    println!("enter a message: ");

    // variabel yang akan menampung inputan user dalam string
    let mut message = String::new();

    // objek reader untuk membaca inputan user
    let stdin_reader = std::io::stdin();

    // proses pembacaan inputan user
    let reader_res = stdin_reader.read_line(&mut message);

    // pengecekan apakah ada error dalam pembacaan inputan.
    // jika iya, maka tampilkan error dan hentikan program
    if reader_res.is_err() {
        println!("error! {:?}", reader_res.err());
        return;
    }

    println!("message: {}", message);
}
