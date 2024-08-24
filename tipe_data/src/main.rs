fn main() {
    // Signed Integers (i8, i16, i32, i64, i128)
    let numerik1 = 32; // default i32 / integer 32
    let numerik2: i8 = 2; // integer 8
    let numerik3: i64 = 12; // integer 64

    println!("Signed Integer");
    println!("{}, | {}, | {}", numerik1, numerik2, numerik3);

    let min_i8 = i8::MIN;
    let max_i8 = i8::MAX;

    println!("MIN int8: {}, MAX int8: {}", min_i8, max_i8);
    println!("MIN int16: {}, MAX int16: {}", i16::MIN, i16::MAX);
    println!("MIN int32: {}, MAX int32: {}", i32::MIN, i32::MAX);
    println!("MIN int64: {}, MAX int64: {}", i64::MIN, i64::MAX);
    println!("MIN int128: {}, MAX int128: {}", i128::MIN, i128::MAX);

    // Unsigned Integers (u8, u16, u32, u64, u128)
    let numerik4: u32 = 28;
    let numerik5: u8 = 16;
    let numerik6: u64 = 42;

    println!("Unsigned Integer");
    println!("{} | {} | {}", numerik4, numerik5, numerik6);

    println!("MIN uint8: {}, MAX uint8: {}", u8::MIN, u8::MAX);
    println!("MIN uint16: {}, MAX uint16: {}", u16::MIN, u16::MAX);
    println!("MIN uint32: {}, MAX uint32: {}", u32::MIN, u32::MAX);
    println!("MIN uint64: {}, MAX uint64: {}", u64::MIN, u64::MAX);
    println!("MIN uint128: {}, MAX uint128: {}", u128::MIN, u128::MAX);

    // Floating point (f32, f64)
    let fp1: f32 = 3.14;
    let fp2: f64 = 3.1415926535;

    println!("Floating Point");
    println!("{} | {:.5}", fp1, fp2);

    println!("MIN float32: {}, MAX float32: {}", f32::MIN, f32::MAX);
    println!("MIN float64: {}, MAX float64: {}", f64::MIN, f64::MAX);

    // Boolean (true, false)
    let b1 = true;
    let b2 = false;

    println!("Boolean");
    println!("{} | {}", b1, b2);

    // Char ('n', '-', '2') / UNICODE
    let c1 = 'n';
    let c2 = '-';
    let c3 = '2';

    println!("Char");
    println!("{} | {} | {}", c1, c2, c3);

    // Pointer Scalar (tambahan karakter &)
    let ptr1: &i32 = &24;
    println!("Pointer Scalar");
    println!("{}", ptr1);

    // String Literal (&str)
    let var1 = "Hello";

    println!("String");
    println!("{}", var1);

    let var2 = "Hello \
    \"rust\" \
    and \
    \"world\""; // escape (\)

    println!("{}", var2);

    let var3 = "baris satu
    baris dua
    baris tiga"; // multiline string literal

    println!("{}", var3);

    let var4 = "baris satu
        baris dua
        baris tiga";

    println!("{}", var4);

    let var5 = r#"
    {
        "name": "ilham kurniawan",
        "gender": "male"
    }
    "#; // raw string (r#""#)

    println!("{}", var5);

    let var6 = "
    {
        \"name\": \"indah kurniawan\",
        \"gender\": \"female\"
    }";

    println!("{}", var6);
}
