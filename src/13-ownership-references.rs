// {{{
#![allow(unused, clippy::ptr_arg)]
// }}}

// NOTE: options to keep string:
// - return it back
// - only get a reference

fn string_length(s: &String) -> usize {
    s.len()
}

// {{{
fn append_name(base: &mut String, name: &String) {
    base.push_str(name);
}
// }}}
// {{{
fn print_my_string(s: &String, len: usize) {
    println!("String \"{}\" has length {}", s, len);
}
// }}}

pub fn main() {
    let mut s = String::from("test");

    append_name(&mut s, &String::from(" dog"));

    // NOTE:                       V hmmm
    let len = string_length(&s);

    println!("String \"{}\" has length {}", s, len);

    // {{{
    // We can have EITHER MULTIPLE readers (non-mut references)
    //                 OR just ONE writer  (    mut reference )
    let ref_1 = &mut s;
    // let ref_2 = &mut s;

    // print_my_string(&s, len);
    append_name(ref_1, &String::from("david"));
    // append_name(ref_2, &String::from("pavel"));
    // }}}
}

// vim: set foldmethod=marker:
