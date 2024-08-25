fn main() {
    // Tuple (tipe data campuran)
    let tuple_a = ("jason", 27, ["racing", "working out"], true);
    println!("tuple_a: {:?}", tuple_a);

    // indexing
    println!("index 0: {:?}", tuple_a.0); // maka indeks 0 akan terakses
    println!("index 1: {:?}", tuple_a.1);
    println!("index 2: {:?}, {:?}", tuple_a.2[0], tuple_a.2[1]); // mengakses indeks ke 2 yaitu sebuah array
    println!("index 3: {:?}", tuple_a.3);

    // mutable tuple
    let mut tuple_b: (&str, i32, [&str; 2], bool) = ("default", 0, [""; 2], false);
    println!("(before) tuple_b: {:?}", tuple_b);

    tuple_b.0 = "ilham";
    tuple_b.1 = 24;
    tuple_b.2 = ["gaming", "adventuring"];
    tuple_b.3 = true;
    println!("(after) tuple_b: {:?}", tuple_b);

    // 4 cara deklarsi tuple
    //// type inference
    let tuple_a = ("jason", 27, ["racing", "working out"], true);
    println!("tuple_a: {:?}", tuple_a);

    //// manifesting typing
    let mut tuple_b: (&str, i32, [&str; 2], bool) = ("default", 0, [""; 2], false);
    tuple_b.0 = "damian";
    tuple_b.1 = 18;
    tuple_b.2 = ["gaming", "adventuring"];
    tuple_b.3 = true;

    //// packing tuple
    let name = "kurniawan";
    let age = 29;
    let hobbies = ["sleeping", "gaming"];

    let tuple_c = (name, age, hobbies);
    println!();
    println!("name: {:?}", tuple_c.0);
    println!("age: {:?}", tuple_c.1);
    println!("hobbies: {:?}, {:?}", tuple_c.2[0], tuple_c.2[1]);

    //// unpacking tuple
    let tuple_d = ("stephanie", 24, ["software engineering"], false);
    let (name, age, hobbies, is_male) = tuple_d;

    println!();
    println!("name: {:?}", name);
    println!("age: {:?}", age);
    println!("hobbies: {:?}", hobbies);
    println!("is_male: {:?}", is_male);
}
