/*
    Block expression (atau cukup block), adalah salah satu control flow yang ada di Rust yang berguna untuk isolasi items,
    variabel, ataupun proses dalam sebuah scope yang sifatnya anonymous.
*/

use rand::Rng;

fn main() {
    let x = 24;
    println!("x: {}", x);

    // block
    {
        println!("(from block) x: {}", x);

        let y = 12;
        let z = x + y;
        println!("(from block) y: {}", y);
        println!("(from block) z: {}", z);
    }

    // return value block
    let a: i32 = {
        let n = rand::thread_rng().gen_range(0..100);
        n * 2 // tanpa return keyword
    };

    println!("a: {}", a);

    // nested block
    {
        let b = 12;
        let mut total: i32;
        {
            let c = 13;
            {
                let d = 14;
                total = b + c + d;
            }
        }

        println!("nested total: {}", total)
    }

    // labeled block
    {
        let mut total = 24;

        // label block
        'append_with_even_number: {
            let n = rand::thread_rng().gen_range(0..100);

            if n % 2 == 1 {
                break 'append_with_even_number;
            }

            total = n
        }

        println!("labeled total: {total}")
    }

    // Note
    /*
        Block biasa diterapkan untuk isolasi sebuah proses yang tidak perlu di-reuse.
        Jika proses adalah di-reuse, dianjurkan untuk menggunakan fungsi dalam penerapannya.

        Di bahasa pemrograman lain juga ada block yang penerapannya kurang lebih adalah sama.
        Namun perlu diketahui, di Rust block memiliki berbedaan yang bisa dibilang signifikan,
        yaitu dalam hal manajemen memory.

        Rust menerapkan konsep memory management bernama ownership.
        Setiap kali Rust selesai mengeksekusi block kode, baik itu fungsi, block expression,
        atau jenis block lainnya; akan dilakukan evaluasi pengecekan ownership yang ada dalam block tersebut.
        Untuk data yang owner-nya tidak berpindah ke luar scope,
        maka akan dilakukan proses dealokasi memory untuk data tersebut.
        Dengan approach ini penggunaan memory menjadi efisien.
    */
}
