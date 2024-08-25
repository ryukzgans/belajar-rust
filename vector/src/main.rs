use std::collections::VecDeque;

fn main() {
    // Vector (vec!)
    let mut data_one = vec!["batman", "superman", "lobo"];

    println!("data: {:?}", data_one);
    println!(
        "length: {}, capacity: {}",
        data_one.len(),
        data_one.capacity()
    );

    // method pop -> menghapus elemen terakhir
    data_one.pop();
    println!("data: {:?}", data_one);
    println!(
        "length: {}, capacity: {}",
        data_one.len(),
        data_one.capacity()
    );

    // method remove -> menghapus elemen index ke I
    data_one.remove(1);
    println!("data: {:?}", data_one);
    println!(
        "length: {}, capacity: {}",
        data_one.len(),
        data_one.capacity()
    );

    // method push -> menambahkan elemen baru
    data_one.push("constantine");
    data_one.push("trigon");
    data_one.push("darkseid");

    println!("data: {:?}", data_one);
    println!(
        "length: {}, capacity: {}",
        data_one.len(),
        data_one.capacity()
    );

    // mengubah value sebuah elemen menggunakan notasi [i]
    data_one[2] = "red hood";
    println!("data: {:?}", data_one);
    println!(
        "length: {}, capacity: {}",
        data_one.len(),
        data_one.capacity()
    );

    // method is_empty -> mengecek apakah vector kosong
    let is_vector_empty = data_one.is_empty();
    println!("is data_one empty: {}", is_vector_empty);

    // method clear -> mengosongkan isi vector
    data_one.clear();
    println!("data: {:?}", data_one);
    println!(
        "length: {}, capacity: {}",
        data_one.len(),
        data_one.capacity()
    );

    // method append -> concatenation/penggabungan vector
    let mut result_one = vec![3, 1, 2];
    let mut data_two = vec![7, 6, 8];

    result_one.append(&mut data_two);
    println!("data: {:?}", result_one);
    println!(
        "length: {}, capacity: {}",
        result_one.len(),
        result_one.capacity()
    );

    result_one.append(&mut vec![4, 5]);
    println!("data: {:?}", result_one);
    println!(
        "length: {}, capacity: {}",
        result_one.len(),
        result_one.capacity()
    );

    // method sort -> untuk mengurutkan vector
    println!("data: {:?}", result_one);
    result_one.sort();
    println!("data: {:?}", result_one);

    // macam-macam deklarasi vector
    let _vector_4 = vec![1, 2, 3];
    let _vector_5: Vec<i64> = vec![4, 5, 6];

    // deklarasi tanpa value
    let _vector_7: Vec<&str> = vec![];
    let _vector_8: Vec<&str> = Vec::new();

    // iterasi pada vector
    let vec_eight = vec![1, 2, 3];
    for e in vec_eight {
        print!("{e}");
    }

    println!();

    let vec_nine = vec![4, 5, 6];
    for i in 0..vec_nine.len() {
        print!("{}", vec_nine[i]);
    }
    println!();

    // ownership tipe data vector
    let vec_ten = vec![1, 2, 3];
    for e in vec_ten {
        print!("{e} ");
    }

    //// error
    // for i in 0..vec_ten.len() {
    //     print!("{} ", vec_ten[i]);
    // }

    //// solusi
    // for e in &vec_ten {
    //     print!("{e} ");
    // }

    println!();

    // vector slice
    let vec_population = vec![2, 1, 3];
    let vec_sample = &vec_population[0..1];
    println!("{:?}", vec_sample);

    // Tipe data VecDeque<T>
    /*
    - method pop_front untuk hapus data elemen pertama atau paling kiri (indeks ke-0)
    - method push_front untuk menambah data dari kiri (indeks ke-0)
    - method pop_back untuk hapus data elemen pertama atau paling kanan (indeks terakhir)
    - method push_back untuk menambah data dari kanan (indeks terakhir)
    */

    let mut vec_10 = VecDeque::from(vec!["a", "b", "c"]);
    println!("(first) vec_10: {:?}", vec_10);

    vec_10.pop_front();
    vec_10.push_front("z");
    println!("(front) vec_10: {:?}", vec_10);

    vec_10.pop_back();
    vec_10.push_back("h");
    println!("(back) vec_10: {:?}", vec_10);
}
