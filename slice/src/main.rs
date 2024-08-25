fn main() {
    // Slice

    // example
    let numbers = [12, 16, 8, 3];
    println!("numbers: {:?}, len: {}", numbers, numbers.len());
    println!("numbers[0] : {:?}", numbers[0]);
    println!("numbers[1] : {:?}", numbers[1]);
    println!();

    let slice_a = &numbers[0..3];
    println!("slice_a: {:?}, len: {}", slice_a, slice_a.len());
    println!("slice_a[0] : {:?}", slice_a[0]);
    println!("slice_a[1] : {:?}", slice_a[1]);
    println!();

    let slice_b = &slice_a[1..=2];
    println!("slice_b: {:?}, len: {}", slice_b, slice_b.len());
    println!("slice_b[0] : {:?}", slice_b[0]);
    println!("slice_b[1] : {:?}", slice_b[1]);

    // Slice range syntax
    let data = ["a", "b", "c", "d"];

    let sliced_data = &data[1..3];
    println!("{:?}", sliced_data); // output >> ["b", "c"]

    let sliced_data = &data[1..=3];
    println!("{:?}", sliced_data); // output >> ["b", "c", "d"]

    let sliced_data = &data[..3];
    println!("{:?}", sliced_data); // output >> ["a", "b", "c"]

    let sliced_data = &data[..=2];
    println!("{:?}", sliced_data); // output >> ["a", "b", "c"]

    let sliced_data = &data[1..];
    println!("{:?}", sliced_data); // output >> ["b", "c", "d"]

    let sliced_data = &data[..];
    println!("{:?}", sliced_data); // output >> ["a", "b", "c", "d"]

    // Mutability pada slice
    /*
        Read only atau shared reference, operator yang digunakan adalah &.
        Mutable reference, operator yang digunakan adalah &mut.
    */
    println!();

    let mut numbers2 = [12, 16, 8, 3];
    println!("======before======");
    println!("numbers 2: {:?}", numbers2);

    let slice_e = &mut numbers2[..=2];
    slice_e[1] = 99;

    println!("======after======");
    println!("slice_e: {:?}", slice_e);
    println!("numbers 2: {:?}", numbers2);

    // Perulangan for in pada slice
    let scores1 = [7, 8, 9];
    for i in &scores1[..] {
        println!("{:?}", i);
    }

    // Perulangna for in pada mutable slice
    let mut scores2 = [7, 8, 9];
    println!("(before) scores2: {:?}", scores2);

    for score in &mut scores2[..] {
        *score += 1;
    }

    println!("(after) scores2: {:?}", scores2);

    /*
        Summary
        - Slice memiliki notasi &[T]
        - & di situ artinya adalah operasi borrowing/peminjaman
        - T adalah tipe data tiap elemen
        - Slice bisa terbentuk dari hasil meminjam data array, vector, atau tipe data kolektif lainnya
        - Data slice adalah selalu data pinjaman
        - Slice memiliki lebar/size
        - Slicing adalah cara pengaksesan slice menggunakan range syntax
        - Slice bisa immutable, bisa juga mutable (menggunakan &mut)
    */
}
