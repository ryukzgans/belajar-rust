// Default visibility untuk hampir semua item adalah private

mod messaging;

pub mod outer_mod {
    pub mod inner_mod {
        // fungsi say_hello berikut hanya bisa diakses dari dalam 'outer_mod'
        // pengaksesannya dari luar 'outer_mod' menghasilkan error
        pub(in crate::outer_mod) fn say_hello1() {
            println!("hello rust");
        }

        // fungsi ini visibility scopenya di level crate
        pub(crate) fn say_hello2() {
            println!("hello rust 2");
        }

        // fungsi ini visibility scope-nya di parent module scope
        // yaitu 'outer_mod'
        pub(super) fn say_hello3() {
            println!("hello rust 3");
        }

        // fungsi ini visibility scope-nya di current module scope
        // yaitu 'inned_mod'
        pub(self) fn say_hello4() {
            println!("hello rust 4");
        }

        pub fn do_something() {
            say_hello4(); // hanya bisa diakses di inner_mod;
        }
    }

    pub fn do_something() {
        inner_mod::say_hello1();
        inner_mod::say_hello2();
        inner_mod::say_hello3();
        // inner_mod::say_hello4(); // gak bisa
    }
}

pub mod outer_mod_two {
    pub fn do_something() {
        crate::outer_mod::inner_mod::say_hello2();
    }
}

fn main() {
    messaging::say_hello();

    // example (in crate)
    outer_mod::do_something();

    // example (crate)
    outer_mod::inner_mod::say_hello2();
    outer_mod::do_something();
    outer_mod_two::do_something();
}
