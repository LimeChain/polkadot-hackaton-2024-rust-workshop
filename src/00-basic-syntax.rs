#![allow(dead_code, clippy::never_loop, unused_variables)]

fn print_hello(num: i32) {
    println!("Hello world, my favourite number is {}", num);
}

#[allow(clippy::needless_return)]
fn get_num() -> i32 {
    println!("Let's see now...");

    // return 5;

    // Booleans - true/false
    if true {
        5
    } else {
        6
    }
}

fn show_numbers() {
    for i in 0..16 {
        println!("i is {}", i);
    }
}

pub fn main() {
    // Integer types                                   VVVVV size of pointers
    // unsigned (0 and up)        - u8, u16, u32, ..., usize
    // signed (includes negative) - i8, i16, i32, ..., isize
    let a: i32 = get_num();

    print_hello(a);

    // Infinite loop
    loop {
        let x = 1 + 2;

        // Unless we break
        break;
    }
    while false {
        // There's also a while loop
    }

    show_numbers();
}
