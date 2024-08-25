fn main() {
    // Array
    let mut numbers = [24, 12, 32, 7];
    println!("numbers: {:?}", numbers); // (:?, :#?)

    let data0 = numbers[0];
    println!("elemen array ke 0 {data0}");

    let data1 = numbers[1];
    println!("elemen array ke 1 {data1}");

    numbers[1] = 16;
    numbers[3] = 8;

    println!("array {numbers:?}");

    // more example array
    let alphabets = ["a", "b", "c", "d"];
    let booleans = [true, false];
    let floating_numbers = [32.0000078, 3.14, 0.5];

    println!("alphabets: {:#?}", alphabets);
    println!("booleans: {:#?}", booleans);
    println!("floating_numbers: {:#?}", floating_numbers);

    // Deklarasi array dengan metode type inference (tanpa define tipe data)
    let angka_integer = [24, 12, 32, 7];
    let angka_float = [24.2, 12.5, 32.00002, 7.2];
    println!("{angka_integer:?}");
    println!("{angka_float:?}");

    // Deklarasi array dengan metode manifest typing disertai predefined value
    // let variabel: [type; size] = [value, value...]
    let data_boolean: [bool; 2] = [true, false];
    println!("{data_boolean:?}");

    let angka_unsigned_integer: [u32; 3] = [24, 0, 12];
    println!("{angka_unsigned_integer:#?}");

    // Deklarasi array dengan notasi penulisan [T; N] (T adalah tipe data elemen, dan N adalah lebar/size array)
    let data_numerik1: [i32; 10] = [0; 10];
    println!("{data_numerik1:?}"); // output >> [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

    let data_numerik2 = [4; 5];
    println!("{data_numerik2:?}"); // output >> [4, 4, 4, 4, 4]

    // len
    let names = ["jason", "grayon", "drake", "damian"];
    let length_names = names.len();
    println!("array size is: {length_names}");

    // iterasi array dengan for in
    for name in names {
        println!("{name}");
    }

    for i in 0..names.len() {
        println!("array index ke-{} = {}", i, names[i]);
    }

    // iterasi array dengan while
    let mut i = 0;
    while i < names.len() {
        println!("array index ke-{}: {}", i, names[i]);
        i += 1;
    }

    // iterasi array dengan loop
    let mut i = 0;
    loop {
        if i >= names.len() {
            break;
        }

        println!("array index ke-{}: {}", i, names[i]);
        i += 1;
    }

    // iterasi array menggunakan for in dan tuple
    for (i, name) in names.iter().enumerate() {
        println!("array index ke-{} = {}", i, name);
    }

    // nested array
    let data_arr = [
        ["salad", "fried rice"],
        ["apple", "coconut"],
        ["spinach", "jalapeno"],
    ];

    for sub_arr in data_arr {
        for el in sub_arr {
            println!("{el}")
        }
        println!();
    }

    println!("{}", data_arr[0][1]);
}
