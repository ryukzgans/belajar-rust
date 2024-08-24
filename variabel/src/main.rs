fn main() {
    let mut message_number: i8 = 1; // mut >> agar bisa di replace/diubah
    let message1: &str = "Hello";
    println!("Message Number {} = {}", message_number, message1);

    message_number = 2;
    let message2: &str = "World";
    println!("Message Number {0} = {1}", message_number, message2);

    message_number = 3;
    let message3: &str = "David";
    println!("Message Number {1} = {0}", message3, message_number);

    // Multiple assign variable
    let (var1, var2) = (24, "hello");
    println!("{} {}", var1, var2);

    // Multiple assign dengan tipe data
    let (var3, var4): (i8, i8) = (32, 12);
    println!("{} {}", var3, var4);

    // Multiple assign dengan mutable
    let (var5, mut var6, var7): (i8, i8, i8) = (64, 12, 4);
    println!("{} {}, {}", var5, var6, var7);
    var6 = 14;
    println!("{} {}, {}", var5, var6, var7);

    // Tipe data ditentukan dari value
    let data1 = 24i8; // bisa lgsung setelah value
    let data2 = 64_i32; // bisa juga dengan underscore

    println!("{}, {}", data1, data2);

    // Variabel Shadowing
    let x = 5;
    println!("x: {}", x);

    let x = x + 1;
    println!("x: {}", x);

    // Variabel _
    let _ = x + 3;
}
