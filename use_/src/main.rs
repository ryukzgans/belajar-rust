// use
use std::env::{args, current_dir};

fn main() {
    // lgsung define
    let package_path_one = std::env::current_dir().unwrap();

    // dengan use
    let package_path_two = current_dir().unwrap();

    println!("package_path_one: {:?}", package_path_one);
    println!("package_path_two: {:?}", package_path_two);

    // cargo run hello world
    for i in 1..=args().len() {
        let each_arg = args().nth(i);
        if each_arg != None {
            println!("arg{}: {:?}", i, each_arg.unwrap());
        }
    }

    // mengambil index pertama pada args
    println!("{:?}", args().nth(1).unwrap());
}
