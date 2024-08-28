fn my_func() {
    println!("calling `my_func()`");
}

// Keyword 'self' pada module scope
mod my_mod1 {
    pub fn my_func() {
        println!("calling `my_mod1::my_func()`");
    }

    pub fn run_the_app() {
        println!("calling `my_mod1::run_the_app()`");
        my_func(); // akan memanggil my_func() dalam scope my_mod1
        self::my_func(); // akan memanggil my_func() dalam scope my_mod1
    }
}

// Keyword 'crate' pada module scope
mod my_mod2 {
    pub fn my_func() {
        println!("calling `my_mod::my_func()`");
    }

    pub fn run_the_app() {
        println!("calling `my_mod2::run_the_app()`");
        crate::my_func(); // akan memanggil my_func() dalam scope main
        self::my_func(); // akan memanggil my_func() dalam scope my_mod2
    }
}

// Keyword 'super'
mod my_mod3 {
    pub fn my_func() {
        println!("calling `my_mod3::my_func()`");
    }

    pub mod my_submod3 {
        pub fn my_func() {
            println!("calling `my_mod3::my_submod3::my_func()`");
        }

        pub fn run_the_app() {
            println!("calling `my_mod3::my_submod3::run_the_app()`");
            crate::my_func(); // akan memanggil my_func() dalam scope main
            super::my_func(); // akan memanggil my_func() dalam scope my_mod3
            self::my_func(); // akan memanggil my_func() dalam scope my_submod3
        }
    }
}

// Lanjutan
mod module_a {

    // path item ➜ `module_a::my_func`.
    pub fn my_func() {
        println!("call `module_a::my_func()`");
    }
}

mod module_b {
    // path item ➜ `module_b::submodule_b_one`.
    mod submodule_b_one {

        // path item ➜ `module_b::submodule_b_one::my_func`.
        pub fn my_func() {
            println!("call `module_b::submodule_b_one::my_func()`");
        }
    }

    // path item ➜ `module_b::submodule_b_two`.
    mod submodule_b_two {

        // path item ➜ `module_b::submodule_b_two::my_func`.
        pub fn my_func() {
            println!("call `module_b::submodule_b_two::my_func()`");

            // current module scope adalah module `submodule_b_two`.
            // keyword `super` di sini mengarah ke parent scope, yaitu `module`.
            //
            // statement `super::my_func()` berikut adalah
            // ekuivalen dengan `module_b::my_func()`
            // jika diakses dari crate root.
            super::my_func();
            //
            // statement `super::submodule_b_one::my_func()` berikut adalah
            // ekuivalen dengan `module_b::submodule_b_one::my_func()`
            // jika diakses dari crate root.
            super::submodule_b_one::my_func();
        }
    }

    // path item ➜ `module_b::my_func`.
    // fungsi ini tidak publik, jadi hanya bisa diakses dalam scope module `module_b` saja.
    fn my_func() {
        println!("call `module_b::my_func()`");
    }

    // path item ➜ `module_b::run_all_funcs`.
    pub fn run_all_funcs() {
        // semua fungsi yang didefinisikan akan di call dalam blok kode ini.
        print!("call `my::run_all_funcs()`");

        // keyword `self` merepresentasikan current module scope.
        // menjadikan dua statement berikut adalah ekuivalen:
        // `my_func()` adalah ekuivalen dengan `self::my_func()`.
        my_func();
        self::my_func();

        // current module scope adalah `module_b`.
        // keyword `super` di sini mengarah ke parent scope, yaitu root atau scope paling luar.
        //
        // statement `super::my_func()` berikut adalah
        // memanggil fungsi `my_func` yang ada di crate root,
        // yang deklarasinya satu level dengan fungsi `main`.
        super::my_func();
        //
        // statement `super::module_a::my_func()` berikut adalah
        // memanggil fungsi `my_func` milik module `module_a` yang ada di crate root.
        super::module_a::my_func();
        //
        // module `submodule_b_two` bisa diakses menggunakan self ataupun tidak
        // karena module tersebut merupakan item yang deklarasinya 1 scope dengan fungsi ini,
        // 1 level dengan `run_all_funcs`.
        submodule_b_two::my_func();
        self::submodule_b_two::my_func();
    }
}

fn main() {
    my_mod1::run_the_app();
    println!();

    my_mod2::run_the_app();
    println!();

    my_mod3::my_submod3::run_the_app();
    println!();

    module_b::run_all_funcs();
    println!();
}
