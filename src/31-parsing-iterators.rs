// {{{
#![allow(dead_code, clippy::identity_op, clippy::map_flatten)]
// }}}

use std::num::ParseIntError;

pub fn main() {
    // {{{ Example of "string to int" parsing
    let num_str = String::from("123");
    // let num_str = String::from("1g3");
    let num_or_error: Result<u32, _> = num_str.parse();

    match num_or_error {
        Ok(num) => println!("Num is {}", num),
        Err(error) => println!("Error is \"{}\"", error),
    }
    // }}}

    // {{{ Example of checked addition
    fn max() -> u8 {
        // 255
        u8::MAX
    }

    let value = max() + 0;
    // let value = max() + 50;
    // {{{
    // let value = max().checked_add(50);
    // let value = max().checked_add(0);

    // match value {
    //     Some(n) => todo!(),
    //     None => todo!(),
    // }
    // }}}

    println!("Value is {:?}", value);
    // }}}

    // {{{ Iterator chain
    let input = String::from("1 200\n4 g\n");

    #[derive(Debug)]
    enum MyError {
        ParseError(ParseIntError),
        Overflow(String),
    }
    use MyError::*;

    // {{{
    fn parse_and_add_100(s: &str) -> Result<u8, MyError> {
        let num: u8 = s.parse().map_err(ParseError)?;
        let num_added = num.checked_add(100).ok_or(Overflow(String::from("oops")))?;
        Ok(num_added)
    }
    // }}}

    let res = input
        .lines()
        .flat_map(|line| line.split_whitespace())
        // {{{
        // .map(parse_and_add_100)
        // }}}
        .map(str::parse::<u8>)
        .map(|res| res.map_err(ParseError))
        .map(|res| res.and_then(|n| n.checked_add(100).ok_or(Overflow(String::from("oops")))))
        // {{{
        // .collect::<Result<Vec<_>, _>>();
        // }}}
        .collect::<Vec<Result<_, _>>>();

    println!("Res is {:?}", res);
    // }}}
}

// vim: set foldmethod=marker:
