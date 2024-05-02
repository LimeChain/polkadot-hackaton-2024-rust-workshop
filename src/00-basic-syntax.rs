// {{{
// {{{
#![allow(
    clippy::needless_return,
    clippy::never_loop,
    clippy::nonminimal_bool,
    dead_code,
    unused_variables,
)]
// }}}

fn print_hello(num: i32) {
    println!("Hello world, my favourite number is {}", num);
}

fn get_num() -> i32 {
    println!("Let's see now...");

    // {{{
    if true {
        5
    } else {
        6
    }
    // }}}
}

fn show_numbers() {
    for i in 0..16 {
        println!("i is {}", i);
    }
}
// }}}

pub fn main() {
    // Booleans
    let a: bool = true && false || true;

    // Integer types                                   VVVVV size of pointers
    // unsigned (0 and up)        - u8, u16, u32, ..., usize
    // signed (includes negative) - i8, i16, i32, ..., isize
    let c = get_num();

    // Floating-point types
    // f32, f64, ... (won't go into details about the rest)
    let d: f64 = 1.2;

    print_hello(c);

    // There's also a while loop
    while false {
        println!("Will not print!");
    }

    // Infinite loop (almost same as `while true`, can return values too!)
    loop {
        println!("1 + 2 = 3");

        // Unless we break (with a value, even)
        break;
    };

    show_numbers();
}

// vim: set foldmethod=marker:
