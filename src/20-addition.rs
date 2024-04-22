#![allow(unused, clippy::identity_op)]

fn max() -> u8 {
    // 255
    u8::MAX
}

fn main() {
    // let value = max() + 0;
    // let value = max() + 1;
    let value = max().checked_add(1);
    // let value = max().checked_add(0);

    // match value {
    //     Some(n) => todo!(),
    //     None => todo!(),
    // }

    let kek: Result<u32, _> = "aloda".parse();
    println!("Kek is {:?}", kek);

    println!("Value is {:?}", value);
}
