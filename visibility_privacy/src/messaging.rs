// Re-export item
// pub use the_path as exported_name;
pub use self::service_layer::some_black_magic as say_hello;

const SOME_MESSAGE: &str = "Hello Rust";

mod service_layer {
    pub fn some_black_magic() {
        println!("{}", crate::messaging::SOME_MESSAGE);
    }
}

// pub fn say_hello() {
//     service_layer::some_black_magic();
// }
