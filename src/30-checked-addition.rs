// {{{
#![allow(unused, clippy::identity_op)]
// }}}

fn max() -> u8 {
    // 255
    u8::MAX
}

fn main() {
    let value = max() + 0;
    // let value = max() + 50;
    // let value = max().checked_add(50);
    // let value = max().checked_add(0);

    // {{{
    // match value {
    //     Some(n) => todo!(),
    //     None => todo!(),
    // }
    // }}}

    println!("Value is {:?}", value);
}

// vim: set foldmethod=marker:
