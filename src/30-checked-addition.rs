// {{{
#![allow(dead_code, clippy::identity_op, clippy::map_flatten)]
// }}}

pub fn main() {
    fn max() -> u8 {
        // 255
        u8::MAX
    }

    let value = max() + 0;
    // let value = max() + 50;
    // let maybe_value = max().checked_add(50);
    // let maybe_value = max().checked_add(0);

    // {{{
    // NOTE: match (for `Option`)
    //
    // match maybe_value {
    //     Some(n) => { println!("We got a value, here it is: {}", n) },
    //     None => { println!("We did not get a value, here it isnt:") },
    // }
    // }}}

    println!("Value is {:?}", value);
}

// vim: set foldmethod=marker:
