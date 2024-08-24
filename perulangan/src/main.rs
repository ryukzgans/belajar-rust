use std::{thread::sleep, time::Duration};

fn main() {
    // Perulangan (while, loop, for in)

    // while
    let mut i = 0;
    let max = 5;

    while i < max {
        println!("nilai i: {}", i);
        i += 1;
    }

    // nested while
    let mut i = 0;
    let max = 5;

    while i < max {
        let mut j = 0;
        let max_inner = i;

        while j <= max_inner {
            print!("* ");
            j += 1;
        }

        println!();
        i += 1;
    }

    // using delay
    let mut i = 0;
    let max = 5;
    while i < max {
        println!("nilai = {i}");
        i += 1;

        sleep(Duration::from_secs(1));
    }

    // loop, perulangan tanpa argument
    let mut i = 0;
    let max = 5;

    loop {
        println!("nilai i: {i}");
        i += 1;
        if i > max {
            break;
        }
    }

    // nested loop
    let mut i = 0;
    let max = 5;
    loop {
        let mut j = max;
        let max_inner = i;

        loop {
            print!("* ");
            j -= 1;
            if j < max_inner {
                break;
            }
        }

        println!();

        i += 1;
        if i > max {
            break;
        }
    }

    // continue
    let mut i = 0;
    let max = 15;

    loop {
        i += 1;

        if i % 2 == 1 {
            continue;
        }

        println!("nilai i: {i}");

        if i > max {
            break;
        }
    }

    // loop dengan label
    let mut i = 0;
    let max = 9;

    'mainloop: loop {
        i += 1;
        let mut j = 0;

        loop {
            if i > max {
                break 'mainloop;
            }

            j += 1;
            if j > i {
                break;
            }

            print!("{i} ")
        }

        println!()
    }

    // returning from loop
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("result: {result}");

    // for in
    for i in 0..5 {
        // iterator (a - b-1)
        println!("{i}");
    }

    for i in 0..=5 {
        // iterator (a - b)
        println!("{i}");
    }

    // label for in
    'perulangan: for i in 0..=5 {
        if i > 3 {
            println!("perulangan diberhentikan paksa pada iterasi {i}");
            break 'perulangan;
        }

        println!("{i}");
    }

    // for in array
    let array = ["jason", "grayon", "drake", "damian"];
    for name in array {
        println!("{name}");
    }
}
