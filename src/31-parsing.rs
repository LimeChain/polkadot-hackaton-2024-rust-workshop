// {{{
#![allow(dead_code, clippy::identity_op, clippy::map_flatten)]
// }}}

pub fn main() {
    let num_str = String::from("123");
    // let num_str = String::from("1g3");
    let num_or_error: Result<u32, _> = num_str.parse();

    match num_or_error {
        Ok(num) => println!("Num is {}", num),
        Err(error) => println!("Error is \"{}\"", error),
    }
}

// vim: set foldmethod=marker:
